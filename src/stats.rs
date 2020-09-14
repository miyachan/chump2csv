use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};

use super::Row;

pub trait WriteStat<T: Write> {
    fn write(&self, writer: &mut csv::Writer<T>, im_start: u64) -> csv::Result<()>;
    fn key(row: &Row) -> Option<u64>;
}

#[derive(Debug, Clone)]
pub struct Media {
    pub(super) media_id: u64,
    pub(super) media_hash: String,
    pub(super) media: Option<String>,
    pub(super) preview_op: Option<String>,
    pub(super) preview_reply: Option<String>,
    pub(super) total: u64,
    pub(super) banned: bool,
}

static MEDIA_ID: AtomicU64 = AtomicU64::new(1);

impl From<&Row> for Media {
    fn from(row: &Row) -> Self {
        Self {
            media_id: MEDIA_ID.fetch_add(1, Ordering::AcqRel),
            media_hash: row.media_hash.clone().unwrap(),
            media: row.media_orig.clone(),
            preview_op: match row.op {
                true => row.preview_orig.clone(),
                false => None,
            },
            preview_reply: match row.op {
                true => None,
                false => row.preview_orig.clone(),
            },
            total: 1,
            banned: false,
        }
    }
}

impl std::ops::AddAssign<&Row> for Media {
    fn add_assign(&mut self, row: &Row) {
        if row.has_image() {
            let preview_op = match row.op {
                true => row.preview_orig.as_ref(),
                false => None,
            };
            let preview_reply = match row.op {
                true => None,
                false => row.preview_orig.as_ref(),
            };
            if self.preview_op.is_none() && preview_op.is_some() {
                self.preview_op = preview_op.cloned();
            }
            if self.preview_reply.is_none() && preview_reply.is_some() {
                self.preview_reply = preview_reply.cloned();
            }
            self.total += 1;
        }
    }
}

impl<T: Write> WriteStat<T> for Media {
    fn write(&self, writer: &mut csv::Writer<T>, media_start_index: u64) -> csv::Result<()> {
        let idx = if media_start_index == 0 {
            Cow::Borrowed(r"\N")
        } else {
            Cow::Owned((self.media_id + media_start_index - 1).to_string())
        };
        writer.write_record(&[
            idx.as_ref(),
            &self.media_hash,
            self.media.as_ref().map(|x| x.as_str()).unwrap_or(r"\N"),
            self.preview_op
                .as_ref()
                .map(|x| x.as_str())
                .unwrap_or(r"\N"),
            self.preview_reply
                .as_ref()
                .map(|x| x.as_str())
                .unwrap_or(r"\N"),
            self.total.to_string().as_str(),
            "0",
        ])
    }

    fn key(row: &Row) -> Option<u64> {
        row.media_hash.as_ref().map(|media_hash| {
            let mut s = fnv::FnvHasher::default();
            media_hash.hash(&mut s);
            s.finish()
        })
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    pub(super) thread_num: usize,
    pub(super) time_op: i64,
    pub(super) time_last: i64,
    pub(super) time_bump: i64,
    pub(super) time_last_modified: i64,
    pub(super) n_replies: u64,
    pub(super) n_images: u64,
    pub(super) sticky: bool,
    pub(super) locked: bool,
}

impl From<&Row> for Thread {
    fn from(row: &Row) -> Self {
        Self {
            thread_num: row.thread_num,
            time_op: row.timestamp,
            time_last: row.timestamp,
            time_bump: row.timestamp,
            time_last_modified: row.timestamp,
            n_replies: 1,
            n_images: if row.has_image() { 1 } else { 0 },
            sticky: row.sticky,
            locked: row.locked,
        }
    }
}

impl std::ops::AddAssign<&Row> for Thread {
    fn add_assign(&mut self, row: &Row) {
        if self.thread_num == row.thread_num {
            if row.op {
                self.time_op = row.timestamp;
                self.sticky = row.sticky;
                self.locked = row.locked;
            }
            self.time_last = self.time_last.max(row.timestamp);
            self.time_bump = if row.email.as_ref().map(|x| x != "sage").unwrap_or(true) {
                self.time_bump.max(row.timestamp)
            } else {
                self.time_bump
            };
            self.time_last_modified = self.time_last.max(row.timestamp);
            self.n_replies += 1;
            self.n_images += if row.has_image() { 1 } else { 0 };
        }
    }
}

impl<T: Write> WriteStat<T> for Thread {
    fn write(&self, writer: &mut csv::Writer<T>, _: u64) -> csv::Result<()> {
        writer.write_record(&[
            self.thread_num.to_string().as_str(),
            self.time_op.to_string().as_str(),
            self.time_last.to_string().as_str(),
            self.time_bump.to_string().as_str(),
            self.time_last_modified.to_string().as_str(),
            self.n_replies.to_string().as_str(),
            self.n_images.to_string().as_str(),
            if self.sticky { "1" } else { "0" },
            if self.locked { "1" } else { "0" },
        ])
    }

    fn key(row: &Row) -> Option<u64> {
        Some(row.thread_num as u64)
    }
}

impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.thread_num.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub(super) name: String,
    pub(super) trip: String,
    pub(super) first_seen: i64,
    pub(super) post_count: i64,
}

impl From<&Row> for User {
    fn from(row: &Row) -> Self {
        Self {
            name: row.name.clone().unwrap_or(String::from("")),
            trip: row.trip.clone().unwrap_or(String::from("")),
            first_seen: row.timestamp,
            post_count: 1,
        }
    }
}

impl std::ops::AddAssign<&Row> for User {
    fn add_assign(&mut self, row: &Row) {
        let empty = String::from("");
        if row.name.as_ref().unwrap_or(&empty) == &self.name
            && row.trip.as_ref().unwrap_or(&empty) == &self.trip
        {
            self.first_seen = self.first_seen.min(row.timestamp);
            self.post_count += 1;
        }
    }
}

impl<T: Write> WriteStat<T> for User {
    fn write(&self, writer: &mut csv::Writer<T>, _: u64) -> csv::Result<()> {
        writer.write_record(&[
            self.name.as_str(),
            self.trip.as_str(),
            self.first_seen.to_string().as_str(),
            self.post_count.to_string().as_str(),
        ])
    }

    fn key(row: &Row) -> Option<u64> {
        if (row.name.is_none() || row.name.as_ref().unwrap().len() == 0)
            && (row.trip.is_none() || row.trip.as_ref().unwrap().len() == 0)
        {
            return None;
        }
        let mut s = fnv::FnvHasher::default();
        match row.name.as_ref() {
            Some(name) => name.hash(&mut s),
            None => (),
        };
        ":".hash(&mut s);
        match row.trip.as_ref() {
            Some(trip) => trip.hash(&mut s),
            None => (),
        };
        Some(s.finish())
    }
}

#[derive(Debug, Clone)]
pub struct Daily {
    pub(super) day: i64,
    pub(super) posts: u64,
    pub(super) images: u64,
    pub(super) sage: u64,
    pub(super) anons: u64,
    pub(super) trips: u64,
    pub(super) names: u64,
}

impl From<&Row> for Daily {
    fn from(row: &Row) -> Self {
        Self {
            day: (row.timestamp / 86400) * 86400,
            posts: 1,
            images: if row.has_image() { 1 } else { 0 },
            sage: row
                .email
                .as_ref()
                .map(|x| if x == "sage" { 1 } else { 0 })
                .unwrap_or(0),
            anons: row
                .name
                .as_ref()
                .map(|x| if x == "Anonymous" { 1 } else { 0 })
                .unwrap_or(0),
            trips: row.trip.as_ref().map(|_| 1).unwrap_or(0),
            names: row
                .name
                .as_ref()
                .map(|x| if x == "Anonymous" { 0 } else { 1 })
                .unwrap_or(0),
        }
    }
}

impl std::ops::AddAssign<&Row> for Daily {
    fn add_assign(&mut self, row: &Row) {
        if self.day == (row.timestamp / 86400) * 86400 {
            self.posts += 1;
            self.images += if row.has_image() { 1 } else { 0 };
            self.sage += row
                .email
                .as_ref()
                .map(|x| if x == "sage" { 1 } else { 0 })
                .unwrap_or(0);
            self.anons += row
                .name
                .as_ref()
                .map(|x| if x == "Anonymous" { 1 } else { 0 })
                .unwrap_or(0);
            self.trips += row.trip.as_ref().map(|_| 1).unwrap_or(0);
            self.names += row
                .name
                .as_ref()
                .map(|x| if x == "Anonymous" { 0 } else { 1 })
                .unwrap_or(0);
        }
    }
}

impl<T: Write> WriteStat<T> for Daily {
    fn write(&self, writer: &mut csv::Writer<T>, _: u64) -> csv::Result<()> {
        writer.write_record(&[
            self.day.to_string().as_str(),
            self.posts.to_string().as_str(),
            self.images.to_string().as_str(),
            self.sage.to_string().as_str(),
            self.anons.to_string().as_str(),
            self.trips.to_string().as_str(),
            self.names.to_string().as_str(),
        ])
    }

    fn key(row: &Row) -> Option<u64> {
        Some(((row.timestamp / 86400) * 86400) as u64)
    }
}
