import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Eduverse } from "../target/types/eduverse";

import {expect} from "chai";

import {approveLesson, createStudent, createTeacher, initialize, registerLesson, registerSubject} from "./instruction";
import {
  deriveConfig, deriveLesson,
  deriveStudentById,
  deriveStudentProfile,
  deriveSubjectConfig, deriveSubjectToTeacher,
  deriveTeacherById,
  deriveTeacherProfile
} from "./pda";

describe("eduverse", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Eduverse as Program<Eduverse>;


  const SUBJECT_ONE: number = 42;
  const SUBJECT_TWO: number = 1337;

  // Some wallets we will use for testing
  const accAdmin = (program.provider as anchor.AnchorProvider).wallet;
  const accAlice = anchor.web3.Keypair.generate();
  const accBob = anchor.web3.Keypair.generate();
  const accCharlie = anchor.web3.Keypair.generate();

  // Prepare some PDAs
  const [accConfig, bumpConfig] = deriveConfig(program);
  const [accTeacherProfileAlice, bumpTeacherProfileAlice] = deriveTeacherProfile(program, accAlice.publicKey);
  const [accTeacherById0, bumpTeacherById0] = deriveTeacherById(program, 0);
  const [accStudentProfileBob, bumpStudentProfileBob] = deriveStudentProfile(program, accBob.publicKey);
  const [accStudentById0, bumpStudentById0] = deriveStudentById(program, 0);
  const [accSubjectConfigOne, bumpSubjectConfigOne] = deriveSubjectConfig(program, SUBJECT_ONE);
  const [accSubjectOneToTeacher0, bumpSubjectOneToTeacher0] = deriveSubjectToTeacher(program, SUBJECT_ONE, 0);
  const [accTeacherAliceLesson1, bumpTeacherAliceLesson1] = deriveLesson(program, accTeacherProfileAlice, 1);


  it("App Airdrops & Initializes", async () => {
    const airdrop1 = await program.provider.connection.requestAirdrop(accAlice.publicKey, 100_000_000);
    await program.provider.connection.confirmTransaction(airdrop1);

    const airdrop2 = await program.provider.connection.requestAirdrop(accBob.publicKey, 100_000_000);
    await program.provider.connection.confirmTransaction(airdrop2);

    const airdrop3 = await program.provider.connection.requestAirdrop(accCharlie.publicKey, 100_000_000);
    await program.provider.connection.confirmTransaction(airdrop3);

    const config = await initialize(program, accAlice, accConfig);
    expect(config).to.not.be.undefined;
  });

  it("Alice can create a teacher profile", async () => {
    const teacher = await createTeacher(program, accConfig, accAlice, accTeacherProfileAlice, accTeacherById0);
    expect(teacher).to.not.be.undefined;
  });

  it("Bob can create a student profile", async () => {
    const student = await createStudent(program, accConfig, accBob, accStudentProfileBob, accStudentById0);
    expect(student).to.not.be.undefined;
  });

  it("Alice can register a subject", async () => {
    const subjectToTeacher = await registerSubject(program, accAlice, accTeacherProfileAlice, accSubjectConfigOne, accSubjectOneToTeacher0, SUBJECT_ONE);
    expect(subjectToTeacher).to.not.be.undefined;
  });

  it("Bob can schedule a lesson with Alice for a subject Alice teaches", async () => {
    // Schedule time
    let next_hour = Math.floor(Date.now() / 1000) + 3600;

    // Does not work for a subject not taught by Alice
    let lessonAlice1 = await registerLesson(program, accAlice, accTeacherById0, accTeacherProfileAlice, accStudentById0, accStudentProfileBob, accTeacherAliceLesson1, 0, 0, SUBJECT_TWO, new anchor.BN(100_000), 60, new anchor.BN(next_hour), "This teacher does not teach the specified subject");
    expect(lessonAlice1).to.be.undefined;

    // Charlie can not create a lesson for Bob taught by Alice
    lessonAlice1 = await registerLesson(program, accCharlie, accTeacherById0, accTeacherProfileAlice, accStudentById0, accStudentProfileBob, accTeacherAliceLesson1, 0, 0, SUBJECT_ONE, new anchor.BN(100_000), 60, new anchor.BN(next_hour), "A raw constraint was violated");
    expect(lessonAlice1).to.be.undefined;

    // Does work for a subject taught by Alice
    lessonAlice1 = await registerLesson(program, accBob, accTeacherById0, accTeacherProfileAlice, accStudentById0, accStudentProfileBob, accTeacherAliceLesson1, 0, 0, SUBJECT_ONE, new anchor.BN(100_000), 60, new anchor.BN(next_hour), "");
    expect(lessonAlice1).to.not.be.undefined;
  });

  it("Alice can approve the lesson scheduled by Bob, but Bob can not", async () => {
    // Does not work for Bob
    let lessonAlice1 = await approveLesson(program, accBob, accTeacherById0, accTeacherProfileAlice, accTeacherAliceLesson1, 0, 1, " An address constraint was violated");
    expect(lessonAlice1).to.be.undefined;

    // Does work for Alice
    lessonAlice1 = await approveLesson(program, accAlice, accTeacherById0, accTeacherProfileAlice, accTeacherAliceLesson1, 0, 1, "");
    expect(lessonAlice1).to.not.be.undefined;
  });
});