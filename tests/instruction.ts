import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Eduverse } from "../target/types/eduverse";

export const initialize = async(program: Program<Eduverse>, txPayer: anchor.web3.Signer, accConfig: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .initialize()
            .accounts({
                payer: txPayer.publicKey,
                config: accConfig,
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        console.log("Error: ", e, " TX: ", tx);
        return undefined;
    }

    return await program.account.config.fetch(accConfig);
}

export const createTeacher = async(program: Program<Eduverse>, accConfig: anchor.web3.PublicKey, txPayer: anchor.web3.Signer, accTeacherProfile: anchor.web3.PublicKey, accTeacherById: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .createTeacherProfile("Teacher Name", "website", "telegram", "twitter")
            .accounts({
                payer: txPayer.publicKey,
                config: accConfig,
                teacherProfile: accTeacherProfile,
                teacherById: accTeacherById
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        console.log("Error: ", e, " TX: ", tx);
        return undefined;
    }

    return await program.account.teacher.fetch(accTeacherProfile);
}

export const createStudent = async(program: Program<Eduverse>, accConfig: anchor.web3.PublicKey, txPayer: anchor.web3.Signer, accStudentProfile: anchor.web3.PublicKey, accStudentById: anchor.web3.PublicKey) => {
    let tx;
    try {
        tx = await program.methods
            .createStudentProfile("Student Name", "Contact Info")
            .accounts({
                payer: txPayer.publicKey,
                config: accConfig,
                studentProfile: accStudentProfile,
                studentById: accStudentById
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        console.log("Error: ", e, " TX: ", tx);
        return undefined;
    }

    return await program.account.student.fetch(accStudentProfile);
}

export const registerSubject = async(program: Program<Eduverse>, txPayer: anchor.web3.Signer, accTeacherProfile: anchor.web3.PublicKey, accSubjectConfig: anchor.web3.PublicKey, accSubjectTeacher: anchor.web3.PublicKey, subjectId: number) => {
    let tx;
    try {
        tx = await program.methods
            .teacherRegisterSubject(subjectId)
            .accounts({
                payer: txPayer.publicKey,
                teacherProfile: accTeacherProfile,
                subjectConfig: accSubjectConfig,
                subjectTeacher: accSubjectTeacher
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        console.log("Error: ", e, " TX: ", tx);
        return undefined;
    }

    return await program.account.subjectTeacher.fetch(accSubjectTeacher);
}

export const registerLesson = async(program: Program<Eduverse>, txPayer: anchor.web3.Signer, accTeacherById: anchor.web3.PublicKey, accTeacherProfile: anchor.web3.PublicKey, accStudentById: anchor.web3.PublicKey, accStudentProfile: anchor.web3.PublicKey, accLesson: anchor.web3.PublicKey, teacherId: number, studentId: number, subjectId: number, fee: anchor.BN, duration: number, dateTime: anchor.BN, expectedError: String) => {
    let tx;
    try {
        tx = await program.methods
            .lessonCreate(teacherId, studentId, subjectId, fee, duration, dateTime)
            .accounts({
                payer: txPayer.publicKey,
                teacherById: accTeacherById,
                teacherProfile: accTeacherProfile,
                studentById: accStudentById,
                studentProfile: accStudentProfile,
                lesson: accLesson
            })
            .signers([txPayer])
            .rpc();
    }
    catch (e) {
        if ( expectedError === "" || !e.toString().includes(expectedError) )
        {
            console.log("Error: ", e, " TX: ", tx);
        }
        return undefined;
    }

    return await program.account.lesson.fetch(accLesson);
}