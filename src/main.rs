use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
struct Activity {
    timestamp: DateTime<Utc>,
    hours: f32,
    description: String,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "habit-tracker", about = "Track your daily improvements")]
enum Command {
    #[structopt(name = "log")]
    Log {
        #[structopt(help = "Hours spent")]
        hours: f32,
        #[structopt(help = "Activity description")]
        description: String,
    },
    #[structopt(name = "view")]
    View {
        #[structopt(help = "Number of days to show", default_value = "365")]
        days: i64,
    },
}

struct HabitTracker {
    data_file: PathBuf,
    activities: Vec<Activity>,
}

impl HabitTracker {
    fn new() -> Self {
        let mut data_dir = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        data_dir.push("habit-tracker");
        fs::create_dir_all(&data_dir).unwrap();

        let data_file = data_dir.join("activities.json");
        // println!("Data file location: {:?}", data_file); // Uncomment this to see the path of your file (Quite a feature, huh?)
        let activities = if data_file.exists() {
            let content = fs::read_to_string(&data_file).unwrap();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        HabitTracker {
            data_file,
            activities,
        }
    }

    fn log_activity(&mut self, hours: f32, description: String) {
        let activity = Activity {
            timestamp: Utc::now(),
            hours,
            description,
        };
        self.activities.push(activity);
        self.save();
    }

    fn save(&self) {
        let content = serde_json::to_string_pretty(&self.activities).unwrap();
        fs::write(&self.data_file, content).unwrap();
    }

    fn view_contributions(&self, days: i64) {
        let now = Local::now();
        let start_date = now - Duration::days(days);

        // Group activities by date
        let mut daily_hours: HashMap<String, f32> = HashMap::new();
        for activity in &self.activities {
            let local_time = activity.timestamp.with_timezone(&Local);
            let date = local_time.format("%Y-%m-%d").to_string();
            *daily_hours.entry(date).or_insert(0.0) += activity.hours;
        }

        // Calculate statistics
        let total_hours: f32 = daily_hours.values().sum();
        let active_days = daily_hours.len();
        let avg_hours = if active_days > 0 {
            total_hours / active_days as f32
        } else {
            0.0
        };
        let max_hours = daily_hours.values().fold(0.0f32, |a, &b| a.max(b));

        // Print header
        println!("\n{}", "📊 Activity Dashboard".bold());
        println!("{}\n", "=================".bold());

        // Print current streak
        let mut streak = 0;
        let mut current = now;
        while current >= start_date {
            let date_str = current.format("%Y-%m-%d").to_string();
            if daily_hours.contains_key(&date_str) {
                streak += 1;
            } else {
                break;
            }
            current = current - Duration::days(1);
        }

        // Print quick stats
        println!("📈 Quick Stats");
        println!("  • Current streak: {} days", streak.to_string().yellow());
        println!(
            "  • Total hours: {} hrs",
            format!("{:.1}", total_hours).green()
        );
        println!(
            "  • Active days: {} of {} days",
            active_days.to_string().cyan(),
            days.to_string().cyan()
        );
        println!(
            "  • Daily average: {} hrs",
            format!("{:.1}", avg_hours).blue()
        );
        println!("  • Best day: {} hrs\n", format!("{:.1}", max_hours).red());

        // Print last 7 days activity
        println!("📅 Last 7 Days");
        for i in (0..7).rev() {
            let date = now - Duration::days(i);
            let date_str = date.format("%Y-%m-%d").to_string();
            let hours = daily_hours.get(&date_str).unwrap_or(&0.0);
            let bar_length = ((hours / max_hours) * 20.0) as usize;
            let bar = "█".repeat(bar_length);
            let weekday = date.format("%a").to_string();

            println!(
                "  {} {} {:<5.1}h {}",
                weekday.bold(),
                date.format("%d/%m").to_string().dimmed(),
                hours,
                if *hours > 0.0 {
                    bar.blue() // Simplified from gradient to single color
                } else {
                    "·".dimmed().to_string().red()
                }
            );
        }

        // Print monthly summary
        println!("\n📊 Monthly Overview");
        let mut month_hours: HashMap<String, f32> = HashMap::new();
        for (date, hours) in daily_hours {
            let naive_date =
                NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date), "%Y-%m-%d %H:%M:%S")
                    .unwrap();
            let month = naive_date.format("%B %Y").to_string();
            *month_hours.entry(month).or_insert(0.0) += hours;
        }

        let max_month_hours = month_hours.values().fold(0.0f32, |a, &b| a.max(b));
        for (month, hours) in month_hours.iter().take(3) {
            let bar_length = ((hours / max_month_hours) * 30.0) as usize;
            let bar = "█".repeat(bar_length);
            println!(
                "  {:<12} {:<5.1}h {}",
                month,
                hours,
                bar.blue() // Simplified from gradient to single color
            );
        }
        println!();
    }
}

fn main() {
    let mut tracker = HabitTracker::new();
    let cmd = Command::from_args();
    match cmd {
        Command::Log { hours, description } => {
            tracker.log_activity(hours, description);
            println!("Activity logged successfully!");
        }
        Command::View { days } => {
            tracker.view_contributions(days);
        }
    }
}
