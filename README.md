# Solana Hackerhouse 2023 Berlin

## Flow

* User connects his wallet (maybe webauth) (optional sns)
* User can create a profile
* User sees a list of teachers (based on filters)
* Teacher profile contains contact data on how to reach (maybe add dialect support later)
* Teacher profile contains lesson rate (per hour/minute/second)
* Teacher creates lesson offer on chain for student to book (with time, duration, rate, tag)
* Teacher profile contains also a introductory video (ipfs or youtube)
* Student deposists required sol into contract, contract holds sol until lesson is over or can cancel based on the agreed upon rules
* User can book a teacher for a lesson
* booking happens onchain
* lesson starts and then per second is paid in sol
* Contract pays out based on lesson rate and lesson duration
* After a lesson is completed the user can review the teacher by giving 1-5 stars and a comment (saved offchain maybe ipfs)


## Notes
* Teacher's pay for their own profile data storage


## Components

* frontend website (react, materialui)
* backend (smart contracts on solana using anchor)
    * user profiles
    * reviews
    * statistics
    * ranking
    * (schedule)
* backend
    * video call (agora, twilio)


## Contract Notes

### User Account
May need to have separate accounts for teacher & students anyway
-  1 byte type (user, teacher, ?)
- 64 byte profile name
-    byte contact info


### Offer for a Lesson
-  8 byte DATE
-  4 byte SUBJECT ID (good for statistics, what was taught)
-  1 byte REPEAT (Daily, Weekly, Monthly, ?)
- 64 byte RATE in LAMPORT / HOUR