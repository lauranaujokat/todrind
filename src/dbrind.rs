struct TemplateEntry {
    start_time: i64,
    duration: i64,
    name: String,
    description: String,

}


struct DailyTemplate {
    entries: Vec<TemplateEntry>,
    start: i32,
    priority: i32,
}
struct WeeklyTemplate {
    entries: Vec<Option<Vec<TemplateEntry>>>,
    start: i32,
    priority: i32,
}
struct MonthlyTemplate {
    entries: Vec<Option<Vec<TemplateEntry>>>,
    start: i32,
    priority: i32,
}
struct YearlyTemplate {
    entries: Vec<Option<Vec<TemplateEntry>>>,
    start: i32,
    priority: i32,
}
struct TimelyTemplate {
    entries: Vec<Option<Vec<TemplateEntry>>>,
    start: i32,
    priority: i32,
}
