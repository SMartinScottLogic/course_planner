use humantime::Duration;
use log::debug;
use serde::{Deserialize, Serialize};

use std::fmt;

use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    details: CourseDetails,
    stages: Vec<Stage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseDetails {
    id: String,
    name: String,
}

impl Course {
    pub fn new(details: &CourseDetails) -> Self {
        let stages = vec![Stage::new("Serving", "0s")];
        Self {
            stages,
            details: details.to_owned(),
        }
    }

    pub fn add(&mut self, stage: Stage) {
        debug!("Add {:?} to course", stage);
        self.stages.push(stage);
    }

    pub fn details(&self) -> &CourseDetails {
        &self.details
    }

    pub fn stages(&self) -> impl Iterator<Item = Stage> + '_ {
        self.stages
            .iter()
            .cloned()
            .sorted_by_cached_key(|stage| -(stage.duration.as_millis() as i128))
            .tuple_windows()
            .map(|(a, b)| Stage {
                name: a.name.clone(),
                duration: a.duration - b.duration,
            })
    }
}

impl CourseDetails {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_owned();
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stage {
    name: String,
    duration: std::time::Duration,
}

impl Stage {
    pub fn new(name: &str, duration: &str) -> Self {
        let duration = duration
            .parse::<Duration>()
            .map(|d| d.into())
            .unwrap_or_else(|_| std::time::Duration::from_secs(0));
        Self {
            name: name.to_string(),
            duration,
        }
    }

    pub fn chain(links: Vec<Stage>) -> impl Iterator<Item = Stage> {
        links
            .into_iter()
            .rev()
            .scan(std::time::Duration::from_secs(0), |acc, v| {
                let mut v = v;
                v.duration += *acc;
                *acc = v.duration;
                debug!(
                    "{} -> {:?} {}",
                    humantime::format_duration(*acc),
                    v,
                    humantime::format_duration(v.duration)
                );
                Some(v)
            })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn duration(&self) -> humantime::FormattedDuration {
        humantime::format_duration(self.duration)
    }
}

impl<'a> fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -- {}",
            humantime::format_duration(self.duration),
            self.name
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
