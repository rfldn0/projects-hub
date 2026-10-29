//! Values derived from a project once, shared by every page that shows it.

use std::cmp::Reverse;
use std::ops::Range;

use crate::model::{Entry, Project};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Done,
    Now,
    Todo,
}

impl Step {
    pub fn slug(self) -> &'static str {
        match self {
            Step::Done => "done",
            Step::Now => "now",
            Step::Todo => "todo",
        }
    }
}

pub struct View<'a> {
    pub p: &'a Project,
    pub done: usize,
    pub pct: u32,
    /// Newest first.
    pub log: Vec<&'a Entry>,
    first_open: Option<usize>,
}

impl<'a> View<'a> {
    pub fn new(p: &'a Project) -> Self {
        let done = p.milestones.iter().filter(|m| m.done).count();
        let total = p.milestones.len();
        let pct = (done * 100 + total / 2).checked_div(total).unwrap_or(0);
        let mut log: Vec<_> = p.log.iter().collect();
        log.sort_by_key(|e| Reverse(e.date));
        let first_open = p.milestones.iter().position(|m| !m.done);
        View { p, done, pct: pct as u32, log, first_open }
    }

    pub fn total(&self) -> usize {
        self.p.milestones.len()
    }

    pub fn latest(&self) -> Option<&'a Entry> {
        self.log.first().copied()
    }

    pub fn step(&self, i: usize) -> Step {
        if self.p.milestones[i].done {
            Step::Done
        } else if Some(i) == self.first_open {
            Step::Now
        } else {
            Step::Todo
        }
    }

    /// Up to three milestones around the current one: last finished, current, next.
    pub fn focus(&self) -> Range<usize> {
        let total = self.total();
        let anchor = self.first_open.unwrap_or(total);
        let start = anchor.saturating_sub(1).min(total.saturating_sub(3));
        start..(start + 3).min(total)
    }
}

/// Views ordered by most recent activity; projects without a log go last.
pub fn build(projects: &[Project]) -> Vec<View<'_>> {
    let mut views: Vec<_> = projects.iter().map(View::new).collect();
    views.sort_by_key(|v| Reverse(v.latest().map(|e| e.date)));
    views
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Milestone, Status};

    fn project(done: &[bool]) -> Project {
        Project {
            id: "x".into(),
            name: "X".into(),
            desc: String::new(),
            status: Status::Active,
            note: None,
            started: String::new(),
            tags: vec![],
            repo: String::new(),
            demo: String::new(),
            milestones: done.iter().map(|&d| Milestone { title: String::new(), done: d }).collect(),
            log: vec![],
        }
    }

    #[test]
    fn progress_rounds() {
        assert_eq!(View::new(&project(&[true, true, false])).pct, 67);
        assert_eq!(View::new(&project(&[])).pct, 0);
    }

    #[test]
    fn focus_window() {
        let p = project(&[true, true, true, true, true, false, false]);
        let v = View::new(&p);
        assert_eq!(v.focus(), 4..7);
        assert_eq!(v.step(5), Step::Now);
        assert_eq!(v.step(6), Step::Todo);
        assert_eq!(View::new(&project(&[false, false, false, false])).focus(), 0..3);
        assert_eq!(View::new(&project(&[true; 5])).focus(), 2..5);
        assert_eq!(View::new(&project(&[true, false])).focus(), 0..2);
    }
}
