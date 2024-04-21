use chrono::NaiveDate;
use date_time_parser::DateParser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} 'some task [on natural language date]'", args[0]);
        return;
    }

    let input = args[1..].join(" ");

    let task = build_task_text(&input);
    println!("{}", task);
}

fn build_task_text(input: &str) -> String {
    let (task, date_text) = split_on_delimiter(input, "@@");
    let date = date_text.and_then(|d| DateParser::parse(&d));

    format_task(&task, date)
}

fn split_on_delimiter(input: &str, delimiter: &str) -> (String, Option<String>) {
    let parts: Vec<&str> = input.splitn(2, delimiter).collect();

    if parts.len() < 2 {
        (parts[0].to_string(), None)
    } else {
        let first_part = input[..input.find(delimiter).unwrap()].trim().to_string();
        let delimiter_len = delimiter.len();
        let second_part_index = input.find(delimiter).unwrap() + delimiter_len;
        let second_part = input[second_part_index..].trim().to_string();

        (first_part, Some(second_part))
    }
}

fn format_task(task: &str, maybe_date: Option<NaiveDate>) -> String {
    return match maybe_date {
        Some(date) => format!(
            "- [ ] #task {} [[due::{}]]",
            task,
            date.format("%Y-%m-%d").to_string()
        ),
        None => format!("- [ ] #task {}", task),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn format_task_without_date() {
        assert_eq!("- [ ] #task some task", format_task("some task", None));
    }

    #[test]
    fn format_task_with_date() {
        let date = NaiveDate::from_ymd_opt(2021, 12, 24).expect("Invalid date");
        assert_eq!(
            "- [ ] #task some task [[due::2021-12-24]]",
            format_task("some task", Some(date))
        );
    }

    #[test]
    fn test_split_on_delimiter_with_delimiter() {
        let expected = ("some task".to_string(), Some("today".to_string()));
        assert_eq!(split_on_delimiter("some task @@ today", "@@"), expected);
    }

    #[test]
    fn test_split_on_delimiter_without_delimiter() {
        let expected = ("some task and nothing else".to_string(), None);
        assert_eq!(
            split_on_delimiter("some task and nothing else", "@@"),
            expected
        );
    }

    #[test]
    fn test_build_task_text_no_date() {
        assert_eq!("- [ ] #task some task", build_task_text("some task"));
    }

    #[test]
    fn test_build_task_text_with_date() {
        assert_eq!(
            "- [ ] #task some task [[due::2024-04-02]]",
            build_task_text("some task @@ April 2nd 2024")
        );
    }
}
