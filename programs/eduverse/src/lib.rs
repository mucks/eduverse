use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
mod utils;

use crate::instructions::*;

declare_id!("AihTskBQM3txbtFMx4awbZrMLsyiVE17LvBs7hskq1W");

#[program]
pub mod eduverse {
    use super::*;

    /// Initialize the app state
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    /// Create a new teacher profile
    pub fn create_teacher_profile(
        ctx: Context<CreateTeacher>,
        title: String,
        website: String,
        telegram: String,
        twitter: String,
    ) -> Result<()> {
        create_teacher::handler(ctx, title, website, telegram, twitter)
    }

    /// Create a new student profile
    pub fn create_student_profile(
        ctx: Context<CreateStudent>,
        title: String,
        contact_info: String,
    ) -> Result<()> {
        create_student::handler(ctx, title, contact_info)
    }

    /// The teacher can register a new subject that they teach
    pub fn teacher_register_subject(ctx: Context<RegisterSubject>, subject_id: u32) -> Result<()> {
        register_subject::handler(ctx, subject_id)
    }

    /// Either teacher or student can create a lesson
    ///TODO since this will be limited a DoS becomes possible, therefore fees must be paid that teacher can collect - so DoS makes teacher rich without doing anything
    pub fn create_lesson(
        ctx: Context<LessonCreate>,
        student_id: u32,
        subject_id: u32,
    ) -> Result<()> {
        lesson_create::handler(ctx, student_id, subject_id, 0, 0, 0)
    }

    /// The student can deposit funds to a lesson approved by the teacher
    pub fn deposit_lesson_funds(ctx: Context<Deposit>, todo: u32, lesson_todo: u32) -> Result<()> {
        deposit::handler(ctx, todo, lesson_todo)
    }
}
