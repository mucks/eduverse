import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Eduverse } from "../target/types/eduverse";

const textEncoder = new TextEncoder();

export const deriveConfig = (program: Program<Eduverse>) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("config")],
        program.programId
    );

export const deriveTeacherProfile = (program: Program<Eduverse>, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("teacher"), payer.toBuffer()],
        program.programId
    );

export const deriveTeacherById = (program: Program<Eduverse>, id: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("teacher_by_id"), new anchor.BN(id).toArrayLike(Buffer, "le", 4)],
        program.programId
    );

export const deriveStudentProfile = (program: Program<Eduverse>, payer: anchor.web3.PublicKey) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("student"), payer.toBuffer()],
        program.programId
    );

export const deriveStudentById = (program: Program<Eduverse>, id: number) =>
    anchor.web3.PublicKey.findProgramAddressSync(
        [textEncoder.encode("student_by_id"), new anchor.BN(id).toArrayLike(Buffer, "le", 4)],
        program.programId
    );