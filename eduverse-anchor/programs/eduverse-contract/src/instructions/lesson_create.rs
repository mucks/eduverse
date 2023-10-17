use crate::errors;
use crate::state::{Lesson, ProfileById, Student, Teacher};
use crate::utils::LessonState;
use anchor_lang::prelude::*;

#[event]
pub struct LessonCreated {
    teacher_id: u32,
    student_id: u32,
    lesson_id: u32,
    subject_id: u32,
}

#[derive(Accounts)]
#[instruction(
teacher_id: u32,
student_id: u32,
subject_id: u32,
fee: u64,
duration: u16,
date_time: u64,
)]
pub struct LessonCreate<'info> {
    #[account(mut,
    constraint = (payer.key() == teacher_profile.authority || payer.key() == student_profile.authority)
    )]
    pub payer: Signer<'info>,

    #[account(
    seeds = ["teacher_by_id".as_bytes(), &teacher_id.to_le_bytes()],
    bump
    )]
    pub teacher_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = teacher_by_id.profile_key)]
    pub teacher_profile: Box<Account<'info, Teacher>>,

    #[account(
    seeds = ["student_by_id".as_bytes(), &student_id.to_le_bytes()],
    bump,
    )]
    pub student_by_id: Box<Account<'info, ProfileById>>,

    #[account(mut, address = student_by_id.profile_key)]
    pub student_profile: Box<Account<'info, Student>>,

    #[account(
    init,
    seeds = ["lesson".as_bytes(), teacher_profile.key().as_ref(), &teacher_profile.count_lessons.to_le_bytes()],
    bump,
    payer = payer,
    space = Lesson::LEN
    )]
    pub lesson: Box<Account<'info, Lesson>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

/// Either teacher or student can create / schedule a lesson
pub fn handler(
    ctx: Context<LessonCreate>,
    teacher_id: u32,
    student_id: u32,
    subject_id: u32,
    fee: u64,
    duration: u16,
    date_time: u64,
) -> Result<()> {
    let teacher_profile = &mut ctx.accounts.teacher_profile;
    let student_profile = &mut ctx.accounts.student_profile;
    let lesson_id = teacher_profile.count_lessons;

    // Some limit on how far into the future lessons can be scheduled
    let current_time = Clock::get().unwrap().unix_timestamp as u64;
    if (date_time < current_time + 1800) || (date_time > current_time + 5184000) {
        return Err(errors::ErrorCode::LessonScheduleBadDateTime.into());
    } //TODO these should probably be configurable by each teacher individually?

    // Make sure this teacher registered this particular subject
    if !teacher_profile.teaches_subject(subject_id) {
        return Err(errors::ErrorCode::SubjectNotTaught.into());
    }

    // Attempt to register this lesson on the teachers schedule
    //TODO make DoS expensive; increase lesson account size? Make accounts sweepable for teacher. (fake teacher could spam student as well)
    if !teacher_profile.schedule_add(lesson_id) {
        return Err(errors::ErrorCode::ScheduleLimitReachedTeacher.into());
    }

    // Attempt to register this lesson on the students schedule
    if !student_profile.schedule_add(teacher_id, lesson_id) {
        return Err(errors::ErrorCode::ScheduleLimitReachedStudent.into());
    }

    //TODO could check teachers schedule for conflicts; but appears out of scope; teacher might choose 1 student over another if there is a conflict

    // Increase total number of lessons created by teacher
    teacher_profile.count_lessons = teacher_profile
        .count_lessons
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store lesson related data
    let lesson = &mut ctx.accounts.lesson;
    lesson.teacher_id = teacher_id;
    lesson.student_id = student_id;
    lesson.subject_id = subject_id;
    lesson.status_teacher = if *ctx.accounts.payer.key == teacher_profile.authority {
        LessonState::Approved
    } else {
        LessonState::Pending
    };
    lesson.status_student = if *ctx.accounts.payer.key == student_profile.authority {
        LessonState::Approved
    } else {
        LessonState::Pending
    };
    lesson.timestamp = date_time;
    lesson.duration = duration;
    lesson.fee_total = fee;
    lesson.fee_deposited = 0;
    lesson.repeat = 0;
    lesson.cancel = 0;

    emit!(LessonCreated {
        teacher_id,
        student_id,
        lesson_id,
        subject_id,
    });

    Ok(())
}
