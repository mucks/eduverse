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
    pub fn teacher_register_subject(ctx: Context<SubjectRegister>, subject_id: u32) -> Result<()> {
        subject_register::handler(ctx, subject_id)
    }

    /// Either teacher or student can create a lesson
    ///TODO since this will be limited a DoS becomes possible, therefore fees must be paid that teacher can collect - so DoS makes teacher rich without doing anything
    pub fn lesson_create(
        ctx: Context<LessonCreate>,
        teacher_id: u32,
        student_id: u32,
        subject_id: u32,
        fee: u64,
        duration: u16,
        date_time: u64,
    ) -> Result<()> {
        lesson_create::handler(ctx, teacher_id, student_id, subject_id, fee, duration, date_time)
    }

    /// Starts the lesson
    pub fn lesson_begin(
        ctx: Context<LessonBegin>,
        teacher_id: u32,
        lesson_id: u32,
        student_id: u32,
    ) -> Result<()> {
        lesson_begin::handler(ctx, teacher_id, lesson_id, student_id)
    }

    /// Ends the lesson
    pub fn lesson_end(
        ctx: Context<LessonEnd>,
        teacher_id: u32,
        lesson_id: u32,
        student_id: u32,
    ) -> Result<()> {
        lesson_end::handler(ctx, teacher_id, lesson_id, student_id)
    }

    /// The student can deposit funds to a lesson approved by the teacher
    pub fn lesson_deposit(ctx: Context<Deposit>, todo: u32, lesson_todo: u32) -> Result<()> {
        deposit::handler(ctx, todo, lesson_todo)
    }

    /// A teacher can withdraw their accumulated funds
    pub fn teacher_withdraw_funds(ctx: Context<Withdraw>, todo: u32, amount: u64) -> Result<()> {
        withdraw::handler(ctx, todo, amount)
    }
}
