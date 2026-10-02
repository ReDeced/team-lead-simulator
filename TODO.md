# TODO — Календарь, интеграция с Game/GameProject и игровой интерфейс

## Контекст

Сейчас в репозитории есть только экран старта игры:

```
main.rs ──► GameStartMenu ──► DifficultyChooseMenu
                         └──► EmployeesRotationMenu
```

При этом `Game` создаётся в `main.rs:16` и **больше нигде не используется**. Игрового цикла
нет: `Game::next_day()` определён в `types.rs:130`, но **не вызывается нигде**.

### Цель

Построить цепочку экранов и дать календарю доступ к данным игры:

```
GameScreen (основной интерфейс игры)
   └─► CalendarMenu (сетка дней, переключение режимов)
          ├─► DayProgressMenu   (подменю, отдельный класс)
          ├─► DayBugsMenu       (подменю, отдельный класс)
          ├─► DayRotationMenu   (подменю, отдельный класс)
          └─► DayFinanceMenu    (подменю, отдельный класс)
```

Каждое подменю — отдельный файл и отдельный класс. Подменю вызываются **из календаря**
и получают **только чтение** (`&DayReport`). Календарь вызывается **из `GameScreen`**.

---

## Зафиксированные архитектурные решения

| # | Решение | Обоснование |
|---|---------|-------------|
| 1 | `GameScreen` — основной интерфейс игры, создаётся в этой задаче | Без него календарю неоткуда вызываться |
| 2 | Ротация реализуется как реальная механика в `Game`, не как заглушка | В коде ротации не существует вообще |
| 3 | История — снимок состояния за день (`DaySnapshot`) в `Vec` | 30 дней — пренебрежимо мало памяти; дельты усложняют код без выигрыша |
| 4 | 4 режима отображения: Прогресс, Баги, Ротация, Финансы | Согласовано |
| 5 | Подменю — чистые представления: на вход `&DayReport`, на выход `Option<DayReportAction>` | Никаких `format!`-аллокаций и игровой логики в отрисовке |

### Целевая структура модулей

```
src/
  main.rs                    — точка входа, роутинг экранов (Start → Game)
  types.rs                   — домен: Game, GameProject, Finance, DaySnapshot, отчёты
  interface/
    mod.rs
    ui.rs                    — НОВОЕ: UI-примитивы (кнопка, панель, текст)
    game_start_menu.rs       — существующий, + кнопка «Начать игру»
    difficulty_chose_menu.rs — существующий, без изменений
    employees_rotation_menu.rs— существующий, без изменений
    game_screen.rs           — НОВОЕ: основной интерфейс игры
    calendar_menu.rs         — переработка PR #1: сетка + режимы
    day_progress_menu.rs     — НОВОЕ: подменю «Прогресс»
    day_bugs_menu.rs         — НОВОЕ: подменю «Баги»
    day_rotation_menu.rs     — НОВОЕ: подменю «Ротация»
    day_finance_menu.rs      — НОВОЕ: подменю «Финансы»
```

Файлы оставлены плоскими, как сейчас в `interface/`, чтобы не вводить поддиректории
в стиле, которого в репозитории ещё нет. Если файлов станет тесно — вынести в
`interface/calendar/` одной операцией.

---

## Обнаруженные проблемы (найдены при анализе)

Это не «хотел бы», это найденные в коде дефекты. Многие блокируют работу календаря,
поэтому вынесены в Фазу 0.

### B1. Индексы багов ломаются при удалении — риск паники · критично

`types.rs:160`, `types.rs:182`, `types.rs:204`, `types.rs:227` — во всех четырёх ветках:

```rust
project.known_bugs.remove(bug_index);   // удаляет элемент, сдвигает все индексы после него
```

`ManagedEmployee.current_bug` (`types.rs:92`) хранит **индекс** в `known_bugs`.
Если двое сотрудников работают над разными багами, то после удаления одного бага
второй сотрудник получает `current_bug`, указывающий на **другой баг**.

Конкретный сценарий паники: `known_bugs = [A, B, C, D]`, у сотрудника X `current_bug = Some(1)`,
у Y — `Some(2)`. X закрывает баг B → `known_bugs = [A, C, D]`. Y обращается к
`known_bugs[2]` → это уже D. Если багов было три, `known_bugs[2]` — выход за границы → **паника**.

**Решение:** `Bug` получает стабильный `id: BugId`, `current_bug: Option<BugId>`,
а поиск идёт через `iter().position(|b| b.id == id)`.

### B2. `Game.project.employees` всегда пуст · критично

`ManagedEmployee` **нигде не конструируется** — ни в одном файле `src/` нет его создания.
`GameProject::new()` (`types.rs:43`) ставит `employees: Vec::new()`, а `Game::new()`
(`types.rs:123`) не принимает сотрудников.

Следствие: `next_day()` (`types.rs:130`) проходит по пустому вектору и **ничего не делает**.
Ни прогресса, ни багов, ни выгорания. Календару показывать нечего.

### B3. Нанять QA невозможно → баги никогда не раскрываются · критично

`GameStartMenu::new()` (`game_start_menu.rs:29-30`) исключает:

```rust
vec![Position::ManualQA, Position::AutoQA, Position::Sysadmin, Position::DevOps]
```

В `employees.json` по 15 сотрудников каждой позиции, так что в наборе остаются
Backend / Frontend / Mobile / UIUXDesigner. QA нанять нельзя → ветка
`Position::AutoQA | Position::ManualQA` в `next_day()` (`types.rs:235`) недостижима →
`hidden_bugs` не переходят в `known_bugs` → **в игре не появится ни одного бага**.

### B4. Кнопки «Начать игру» нет · блокирует

`game_start_menu.rs:111` — `// TODO: сделать кнопку начала игры`. `draw()` возвращает
`Option<(Difficulty, Vec<Employee>)>`, но `None` наступает всегда: `main.rs:28`
результат игнорирует.

### B5. `Employee.quality` не используется · средне

`types.rs:80` — поле объявлено, но нигде не читается. Влияет на смысл фиксинга багов:
исход сложность бага, решает сотрудник, качество влияет на вероятность ошибки. Сейчас
`Bug.complexity` уменьшается детерминированно.

### B6. `ManagedEmployee.grade` и `education_progress` мертвы · средне

`types.rs:87` и `types.rs:89` — поля объявлены `pub`, но никогда не пишутся и не читаются.
`education_progress` задумывался как обучение с повышением грейда. Либо реализовать,
 либо удалить.

### B7. Нет цели проекта · средне

Пять полей прогресса — неограниченные `f32` (`types.rs:24-28`). «100% прогресса» неоткуда
взять. Для полос в `GameScreen` и для нормализации в календаре нужна цель проекта
(`GameProject::target: ProjectProgress`) либо явный горизонт.

### B8. Мелочи в `next_day()` · низко

- `Position::UIUXDesigner` (`types.rs:232`) игнорирует `current_bug` — дизайнеры не чинят баги.
- `work_done = work_progress.trunc()` (`types.rs:140`) — при `speed < 1.0` сотрудник даёт
  единицу работы раз в несколько дней. Квантование надо задокументировать.
- `Employee.speed`, `quality`, `burnout_coef` — приватные (`types.rs:79-81`), но `speed`
  и `burnout_coef` читаются. Для UI-подменю и отчётов нужно решить, что показываем.

---

## Порядок выполнения

Фазы I и II независимы и могут идти параллельно. Фаза III зависит от I и II.
Фаза IV зависит от III.

```
I (фундамент) ──┬─► II (домен) ──► III (UI) ──► IV (интеграция)
                └─►
```

---

# Фаза I — Фундамент

## T01. Починить учёт бага: `BugId` вместо индексов · B1

**Файлы:** `src/types.rs`

- Добавить `pub struct BugId(u32)` (новостой счётчик счётчик либо `NonZeroU32` + генератор).
- `Bug` получает `pub id: BugId`.
- `ManagedEmployee.current_bug: Option<BugId>` вместо `Option<usize>`.
- В `next_day()` заменить 4 копии логики на один хелпер
  `fn work_on_bug(employee, project, bug_type) -> BugWorkResult`.
- Убрать дублирование: сейчас блок «взять баг / починить баг» скопирован 4 раза
  для Backend, Frontend, Mobile, DevOps+Sysadmin (`types.rs:142-231`).

**Готово, когда:** тест с двумя сотрудниками на разных багах не паникует и каждый
чинит свой баг; `grep -c "project.known_bugs.remove"` даёт 0 в цикле.

## T02. Сделать `ManagedEmployee` конструируемым + найм · B2

**Файлы:** `src/types.rs`

- `ManagedEmployee::new(employee: Employee, days_off: Vec<u8>) -> Self` — инициализация
  полей (сейчас нет конструктора вообще).
- `Game::hire(&mut self, employees: Vec<Employee>)` — превращает `Employee` в `ManagedEmployee`.
- `Game::fire(&mut self, index: usize)`.
- `Game::new(difficulty)` принимает `Vec<Employee>` (или найм вызывается отдельно из `GameScreen`).

**Готово, когда:** `Game` с 4 сотрудниками после `advance_day()` даёт ненулевой прогресс.

## T03. Вернуть QA в набор найма · B3

**Файлы:** `src/interface/game_start_menu.rs`

- Убрать `ManualQA`, `AutoQA` из списка исключений (`game_start_menu.rs:29-30`).
- Взвесить: `Sysadmin`/`DevOps` исключены, но оба деплоят — можно оставить, QA обязательны,
  иначе баги не появляются.
- `hidden_bugs` стартового проекта наполнять при старте игры, иначе и после найма QA
  раскрывать нечего.

**Готово, когда:** нанятый QA в команде → за день `known_bugs` пополняется.

## T04. UI-примитивы · новое

**Файлы:** `src/interface/ui.rs`, `src/interface/mod.rs`

Переиспользовать в 6 новых файлах UI, иначе каждый будет копировать `Rect` + hover + draw.

- `UiStyle { font_size, indent, border_width, bg, fg, accent, hover }` — единый стиль.
- `UiButton { rect: Rect, label: String, enabled: bool }` + `fn draw(&self, font, mouse) -> bool`
  (возвращает `true` при клике). Убирает копипаст `draw_rectangle` / `draw_rectangle_lines` /
  `is_mouse_button_pressed` из существующих меню.
- `UiPanel { rect: Rect }` + `fn draw(&self, font, title: &str)`.
- Хелпер текста без аллокаций в кадре: `draw_label(font, style, text, rect, align)` +
  кэш `format!`-строк на уровне вызывающего структуры.
  Сейчас `employees_rotation_menu.rs:74,87,117,128` делает по 2-4 `format!` на карточку
  за кадр, в `calendar_menu.rs:87` — 30 `to_string()` за кадр.

**Готово, когда:** новые меню не содержат ни одного прямого `draw_rectangle_lines` —
только `UiPanel`/`UiButton`.

---

# Фаза II — Домен

## T05. `Game`: календарное время, финансы, история

**Файлы:** `src/types.rs`

```rust
pub struct Game {
    pub difficulty: Difficulty,
    pub current_day: u8,          // НОВОЕ: 1-based; дней в игре — DAYS_IN_MONTH
    pub project: GameProject,
    pub finance: Finance,         // НОВОЕ
    pub history: Vec<DaySnapshot>,// НОВОЕ
}

pub struct Finance { pub balance: f32 }   // НОВОЕ

impl Game {
    pub const DAYS_IN_MONTH: u8 = 30;
    pub fn weekday(&self) -> u8 { (self.current_day - 1) % 7 }
}
```

- `next_day()` переименовать в `advance_day()`: инкремент `current_day`, запись снимка,
  списание зарплат, проверка конца месяца.
- `history[0]` — стартовое состояние (день 1, всё нулевое), чтобы календарь не был пустым
  на первом дне.
- Добавить `Game::is_finished()` для конца месяца.

**Готово, когда:** 30 вызовов `advance_day()` дают `history.len() == 30`, счётчик достигает 30.

## T06. Типы отчётов — чистое чтение для UI

**Файлы:** `src/types.rs` (или новый `src/reports.rs` — решить по объёму)

```rust
pub struct DaySnapshot {
    pub day: u8,
    pub progress: ProjectProgress,        // абсолютные значения на конец дня
    pub progress_delta: ProjectProgress,   // прирост за день
    pub bugs: BugDayStats,
    pub rotation: Vec<ShiftSnapshot>,
    pub finance: FinanceDayStats,
}

pub struct BugDayStats {
    pub known_before: usize, pub known_after: usize,
    pub hidden_before: usize, pub hidden_after: usize,
    pub opened: usize, pub closed: usize, pub in_progress: usize,
    pub complexity_fixed: f32,
    pub opened_by_type: [usize; BUG_TYPE_COUNT],
    pub closed_by_type: [usize; BUG_TYPE_COUNT],
}

pub struct ShiftSnapshot {
    pub name: String, pub position: Position, pub grade: Grade,
    pub is_working: bool, pub work_done: f32, pub fixed_bug: bool,
}

impl Game {
    pub fn report_for(&self, day: u8) -> Option<&DaySnapshot>;
}
```

- `ProjectProgress` переиспользуется и для цели проекта (B7).
- `BUG_TYPE_COUNT` из `strum::EnumCount` на `BugType` — как уже сделано для `Difficulty`.
- Отчёты — plain data без логики отрисовки.

**Готово, когда:** `report_for(day)` возвращает `None` для `day > current_day`
и для `day == 0`.

## T07. Ротация сотрудников

**Файлы:** `src/types.rs`

- `ManagedEmployee` получает `pub days_off: Vec<u8>` — дни недели (0..7) отдыха.
- `Game::is_working(&self, employee) -> bool` = `!days_off.contains(&weekday)`.
- При найме `days_off` генерируется детерминированно (2 выходных из 7), чтобы
  гарантировать покрытие всех позиций.
- В `advance_day()` для отдыхающих: `burnout` уменьшается (отдых), `work_progress`
  **не растёт** — иначе накопленный прогресс «выстрелит» в первый же рабочий день.
- Штраф за переработку: при 7 рабочих днях подряд — повышенный расход `burnout`.
- `ManagedEmployee.streak: u8` — серия рабочих дней подряд.

**Готово, когда:** у каждого сотрудника есть выходные; отдыхающий за день не даёт прогресса;
`work_progress` не копится во время отдыха.

## T08. Сбор статистики в `advance_day()`

**Файлы:** `src/types.rs`

- Перед циклом снять `known_bugs.len()` / `hidden_bugs.len()`, после — записать дельты.
- `progress_delta` — разница снапшотов пяти полей.
- Баги: `opened` / `closed` / `in_progress` / `complexity_fixed` + разбивка по
  `BugType` (4 счётчика). Идентичность бага не нужна — достаточно счётчиков.
- Финансы: `salary_cost = Σ employee.salary / DAYS_IN_MONTH` только для рабочих,
  списывается с `balance`.
- `ShiftSnapshot` на каждого сотрудника: работал / отдыхал / сколько сделал / закрыл ли баг.
- **Не вызывать `format!`/аллокации в этой функции** — она в горячем пути.

**Готово, когда:** после 10 дней `sum(closed) <= sum(opened)` и
`Σ progress_delta == progress_at_day_10`.

---

# Фаза III — UI

## T09. `DayProgressMenu` — подменю «Прогресс»

**Файлы:** `src/interface/day_progress_menu.rs`

- На вход `&DaySnapshot` + `Rect`. Никаких ссылок на `Game` — только чтение отчёта.
- 5 строк: направление, прирост за день (`+3.0`), абсолютное значение, доля от `target` (B7).
- Полоска прогресса; направления, где прирост = 0, приглушены.
- Кнопка «Закрыть» / возврат по `Esc` → `Some(DayReportAction::Close)`.

**Готово, когда:** показывает все 5 направлений; дельты совпадают с `progress_delta`.

## T10. `DayBugsMenu` — подменю «Баги»

**Файлы:** `src/interface/day_bugs_menu.rs`

- Итоги дня: открыто / закрыто / в работе / осталось сложности.
- Разбивка по 4 типам багов (`BugType`).
- «Было → стало»: `known_before → known_after`, `hidden_before → hidden_after`.
- Список типов, по которым QA не работает (`AutoQA`/`ManualQA` в команде) — в этом
  и причина, почему скрытых багов не становится меньше.

**Готово, когда:** видно динамику по всем четырём типам.

## T11. `DayRotationMenu` — подменю «Ротация»

**Файлы:** `src/interface/day_rotation_menu.rs`

- Таблица сотрудников: имя, позиция, грейд, статус (работал / отдыхал).
- Что сделал за день: объём работы, закрыл ли баг.
- Сгруппировать по позициям — так виднее перекос состава.
- Отметить переработку (`streak >= 6`).

**Готово, когда:** по каждому сотруднику видно состояние на выбранный день,
а не на текущий.

## T12. `DayFinanceMenu` — подменю «Финансы»

**Файлы:** `src/interface/day_finance_menu.rs`

- Баланс до / после / изменение за день.
- Списано зарплат, сколько сотрудников работало, стоимость человеко-дня.
- Средняя стоимость закрытого бага (если будет бонус за фикс).

**Готово, когда:** `balance_after == balance_before - salary_cost`.

## T13. `CalendarMenu` — сетка, режимы, hover

**Файлы:** `src/interface/calendar_menu.rs` (переработка версии из PR #1)

- `pub enum CalendarMode { Progress, Bugs, Rotation, Finance }` — `strum::Display` +
  `EnumIter`, как уже сделано для `Difficulty`.
- Сетка дней 1..=30 из `Game::DAYS_IN_MONTH`. Убрать магические `7`/`6`/`30` в константы.
- Ячейка красится по активному режиму:
  - Прогресс — сумма `progress_delta`, от холодного к горячему
  - Баги — закрыто/открыто
  - Ротация — доля работавших
  - Финансы — расход
- **При hover на день** открывается соответствующее подменю: `mode → подменю`.
- Кнопки переключения режима сверху, кнопка «Закрыть» / `Esc`.
- Дни в будущем (`day > current_day`) — приглушены и некликабельны.
- Заменить `CalendarMenu::selected_day` на `hovered_day` + данные родителя, чтобы
  не дублировать состояние.
- Кнопка «Сегодня» — переход к текущему дню.

**Готово, когда:** смена режима меняет цвет сетки и подменю под курсором;
клик по будущему дню ничего не делает; `format!`/`to_string()` не вызываются в цикле отрисовки.

**Getters:**

```rust
pub fn draw(&mut self, font: &Font, game: &Game) -> CalendarAction
pub enum CalendarAction { None, Close }
```

---

# Фаза IV — Интеграция

## T14. `GameScreen` — основной интерфейс игры

**Файлы:** `src/interface/game_screen.rs`

- Владеет `Game`. Рисует 5 полос прогресса, счётчик «День 7 / 30», баланс,
  список команды с выгоранием.
- Кнопка **«Календарь»** → открывает `CalendarMenu`. Закрытие по `Esc` / кнопке.
- Кнопка **«Конец дня»** → `game.advance_day()`.
- Открытие подменю внутри календаря не трогает состояние `Game`.

**Готово, когда:** из `GameScreen` открывается календарь, «Конец дня» двигает счётчик,
в календаре видна статистика прошедших дней.

## T15. `GameStartMenu` → «Начать игру» · B4

**Файлы:** `src/interface/game_start_menu.rs`, `src/main.rs`

- Добавить состояние `Starting` и кнопку «Начать игру» (`game_start_menu.rs:111`).
- `main.rs`: роутинг экранов, `GameStartMenu::draw()` → `Game::new(difficulty)` + найм
  выбранных сотрудников (T02) → `GameScreen`.
- `Game::new(difficulty)` сейчас не принимает сотрудников — согласовать с T02.

**Готово, когда:** из стартового меню можно начать игру и попасть в `GameScreen`
с выбранной командой.

## T16. Проверка и тесты

**Файлы:** `tests/`, либо модульные тесты в `types.rs`

- `advance_day()` не паникует при двух сотрудниках на разных багах (регресс на B1).
- `Σ progress_delta == progress_delta_total` за диапазон дней.
- `Σ closed <= Σ opened`.
- `balance_after == balance_before - salary_cost` с точностью до `f32`.
- Отдых: за день отдыха `work_progress` не растёт.
- `report_for()` возвращает `None` за пределами пройденных дней.
- Полный проход 30 дней без паники и без новых warnings (`cargo build` — чисто,
  текущие 5 warnings в `master` не должны размножаться).

---

## Сводка

| Задача | Фаза | Зависит от | Закрывает |
|--------|------|-----------|-----------|
| T01 | I | — | B1 |
| T02 | I | — | B2 |
| T03 | I | — | B3 |
| T04 | I | — | — |
| T05 | II | — | B7 (частично) |
| T06 | II | T05 | — |
| T07 | II | T02 | B2 |
| T08 | II | T01, T02, T05, T06, T07 | B5, B8 |
| T09 | III | T04, T06 | — |
| T10 | III | T04, T06 | — |
| T11 | III | T04, T06, T07 | — |
| T12 | III | T04, T06 | — |
| T13 | III | T04, T09–T12 | — |
| T14 | IV | T13 | B4 (часть) |
| T15 | IV | T14, T02 | B4 |
| T16 | IV | T15 | все |

## Порядок взятия в работу

Рекомендуемый порядок, по одной задаче на PR:

1. **T01** — фундамент, ломает всё остальное
2. **T02** + **T03** — без команды и QA игра пустая
3. **T04** — UI-примитивы, дальше они окупаются на каждом экране
4. **T05** → **T06** → **T07** → **T08** — домен целиком
5. **T09** … **T12** — подменю, их можно параллельно
6. **T13** — календарь
7. **T14** → **T15** — интеграция и роутинг
8. **T16** — тесты

## Вне объёма (осознанно)

- Несколько месяцев и переход между ними — только один месяц, 30 дней.
- Сохранение/загрузка игры — `serde`-выводы на месте, но persistence не делаем.
- Рефакторинг `next_day()` в конечный автомат по позициям — попутно уберём
  дублирование в T01, но не более того.
- `Employee.quality` влияет на исход фикса — идея из B5, но реализация после
  того, как заработает цикл.