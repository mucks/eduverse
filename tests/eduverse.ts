import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Eduverse } from "../target/types/eduverse";

import {expect} from "chai";

import {createStudent, createTeacher, initialize, registerSubject} from "./instruction";
import {
  deriveConfig,
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

  // Prepare some PDAs
  const [accConfig, bumpConfig] = deriveConfig(program);
  const [accTeacherProfileAlice, bumpTeacherProfileAlice] = deriveTeacherProfile(program, accAlice.publicKey);
  const [accTeacherById0, bumpTeacherById0] = deriveTeacherById(program, 0);
  const [accStudentProfileBob, bumpStudentProfileBob] = deriveStudentProfile(program, accBob.publicKey);
  const [accStudentById0, bumpStudentById0] = deriveStudentById(program, 0);
  const [accSubjectConfigOne, bumpSubjectConfigOne] = deriveSubjectConfig(program, SUBJECT_ONE);
  const [accSubjectOneToTeacher0, bumpSubjectOneToTeacher0] = deriveSubjectToTeacher(program, SUBJECT_ONE, 0);


  it("App Airdrops & Initializes", async () => {
    const airdrop1 = await program.provider.connection.requestAirdrop(accAlice.publicKey, 100_000_000);
    await program.provider.connection.confirmTransaction(airdrop1);

    const airdrop2 = await program.provider.connection.requestAirdrop(accBob.publicKey, 100_000_000);
    await program.provider.connection.confirmTransaction(airdrop2);

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
});