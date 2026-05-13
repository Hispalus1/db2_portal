use oracle::{Connection, Result};
use crate::orm::dto::review::Review;

pub struct ReviewDao<'a> {
    conn: &'a Connection,
}

impl<'a> ReviewDao<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn get_by_user_and_game(&self, user_id: i32, game_id: i32) -> Result<Option<Review>> {
        let sql = "SELECT review_id, user_id, game_id, rating, review_comment FROM Review WHERE user_id = :1 AND game_id = :2";
        match self.conn.query_row(sql, &[&user_id, &game_id]) {
            Ok(row) => {
                Ok(Some(Review {
                    review_id: row.get(0)?,
                    user_id: row.get(1)?,
                    game_id: row.get(2)?,
                    rating: row.get(3)?,
                    review_comment: row.get(4)?,
                }))
            }
            Err(e) => {
                if e.kind() == oracle::ErrorKind::NoDataFound {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn insert(&self, review: &Review) -> Result<()> {
        let sql = "INSERT INTO Review (user_id, game_id, rating, review_comment) VALUES (:1, :2, :3, :4)";
        self.conn.execute(sql, &[&review.user_id, &review.game_id, &review.rating, &review.review_comment])?;
        Ok(())
    }

    pub fn update(&self, review: &Review) -> Result<()> {
        let sql = "UPDATE Review SET rating = :1, review_comment = :2 WHERE review_id = :3";
        self.conn.execute(sql, &[&review.rating, &review.review_comment, &review.review_id])?;
        Ok(())
    }
}
