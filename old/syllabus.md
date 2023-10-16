# CS 410P/510 Rust: Rust Programming

**Term:** Winter 2022  
**Credits:** 4 (410P), 3 (510)  
**CRN:** 44845 (410P), 44846 (510)  
**Section:** 70 (410P), 91 (510)  
**Meeting Time:** Monday, Wendesday 1440-1630 (4:40-6:30PM)  
**Meeting Location:** Remote (see below)  
**Instructor:** Bart Massey (bart@cs.pdx.edu)  
**TCSS:** Cassaundra Smith (cassaun2@pdx.edu)  
**Office Hours** TBA  
**Prerequisites:** See below

## Disclaimer

*Everything about this syllabus is entirely tentative, and
 may be changed at the whim of the instructor without
 warning.*

## Description

In this course, we will learn to program in Rust.  Rust is a
novel programming language combining solid bare-metal
performance with modern language features that give
acceptable usability and encourage high-quality code.

## Prerequisites

**Required Courses:** CS 201, CS 202

Familiarity with programming in general and C or C++ in
particular is required. Basic ability to use the Linux
environment is required. CS 300 or equivalent is highly
recommended.

## Goals, Topics and Objectives

The languages of choice for systems programming for the past
30 years have been C and its larger cousin C++. Because of
their popularity and performance, C and C++ have also been a
common choice for application programming. However, these
languages are well-understood to be both error-prone,
inconveniently verbose and repetitive, and difficult to use
for large-scale software engineering.

Rust is an attempt to remedy many of the deficiencies of C
and C++. Rust provides a modern strong static type system, a
strict module system supporting separate compilation, and a
static programmer-transparent memory allocation scheme that
largely eliminates runtime memory errors while preserving
the efficiency of manual allocation.

In this course, we will learn to program in Rust. Upon the
successful completion of this course students will be able
to:

* Create a Rust project, including proper Cargo
  configuration.

* Translate a design into a working Rust program.

* Explain and remedy type and lifetime errors encountered
  during Rust programming.

* Construct reasonable types within programs, including
  parametric and lifetime-bounded types.

* Use structs, enums and traits as intended in the
  construction of Rust programs.

* Apply references, boxes, cells and reference counting
  in Rust programming.

* Divide a Rust crate into multiple source files using
  the module system.

* Write tests and documentation using the Rust
  infrastructure.

* Write a Rust package ("crate") that integrates smoothly
  into the Rust ecosystem.

## Course Texts

> *Programming Rust: Fast, Safe Systems Development*  
> Jim Blandy and Jason Orendorff  
> 2nd edition, O'Reilly 2021

You will want and need the second edition of this text: it
has been updated to work with modern Rust, and contains an
additional chapter not in the first edition that will be
covered in the course.

> *The Rust Programming Language*  
> Steve Klabnik, Carol Nichols, et al  
> https://doc.rust-lang.org/book/

While you can buy a paper copy of this book if you like,
most folks prefer to use the free online edition.

## Course Communication

### Remote Course

I will be teaching this course remotely for this
offering. This will be a challenge for all of us. I am
confident we can get a good outcome, but please bear with me
as we work the details out.

To participate effectively in this course, you will need to
have good Internet access, including outside of lecture
hours. If this is a problem for you, please contact me as
soon as possible so we can try to work out an accommodation.

### Communication Tools

Email will be used infrequently. You are required to use
your official `@pdx.edu` email address as registered with the
University to participate in this course. This enables
greater security for online communications.

Communications for this course will be through a Zulip chat
server at http://cs510rust-winter2022.zulip.cs.pdx.edu .
You are required to monitor this channel for course
announcements, etc and are strongly encouraged to
participate. In addition, you must monitor your `@pdx.edu`
email for things that cannot go through Zulip.

Everyone is required to be present in the Zulip.  Please use
Zulip for discussion with other students, and as the
first-choice place to message me if you want to discuss. I
should be available pretty solidly on Zulip during the
quarter.  However, I'd encourage you to post to the course
Zulip channel if possible, as some other student may get to
your question quicker.

This course will use the official PSU Canvas "Learning
Management System." This is the first quarter of Canvas at
PSU as a whole, although I used it for a course here last
quarter. Please be patient and ask questions as we get
started with a new LMS.

Look for invites to Zulip and Moodle in your `@pdx.edu`
email at the start of the quarter.

## Lectures

The course lectures will cover a variety of topics, and may
include guest lectures. Please attend them all; they are
required and should be useful.

Bi-weekly "in person" lectures will be starting at the
listed course time using the [Zoom](http://pdx.zoom.us)
meeting tool: you must be logged into `pdx.zoom.us` to
access them. While it is possible that some lectures will
run the full two hours, a lecture time of about one hour
will be normal. The Zoom lectures will be recorded for the
convenience of students in this difficult time. However,
online attendance is expected if at all possible, and may
very occasionally be required. Please do not schedule this
course in such a way that you will not ever be able to
attend (for example, in conflict with another class or work
schedule).


Lecture time will be focused on demonstrations and
discussion.  In addition, short "pre-recorded" video
lectures will be posted to PSU Media Space
<https://media.pdx.edu/> that you should watch before each
"in-person" lecture.

## Course Work

### Workload

This course requires substantial out-of-class homework and
study.  Expect to spend at least 8 hours of out-of-class
time each week mastering this difficult material.

I encourage group collaboration on individual assignments:
using the course chat or creating online chat-room study
groups to discuss the approach and understand the problem is
encouraged. The write-up, programming, and actual solutions
must be your individual work. If you represent someone
else's work as your own, you are committing plagiarism (see
below).

### Homework

I will assign homework many weeks. Homework assignments are
currently planned to count for about 60% of your course
grade; the other 40% being for the project. This may change,
however. No quizzes or exams are planned or expected.

You may submit a homework as many times as you like, with
the latest assignment received before submission closes
being the only one considered for a grade. Please submit
something before the assignment due date, even if it is only
your name, so that you do not fail the course (see
below). You can then continue to work on your assignment as
desired up until it is graded. Submission may close at the
assignment due date: assignments first received after
submission closes will likely receive a score of zero and
may result in course failure.

Assignments will be graded for having been turned in and
having made a reasonable effort, as well as for a reasonable
degree of correctness.

It is important to keep up with this course. *A score of zero
on any assignment will result in a course grade of F, unless
excused by me for exceptional circumstances.*

### Course Project

You will be assigned a substantial individual course project
around week four, which will continue throughout the latter
part of the quarter.

#### Laboratory

Currently, the CS laboratory facilities consist of machines
in the Linux Lab and Windows Lab. Work may be done remotely
on these or other Departmental boxes. Those with home or
laptop boxes are encouraged to use them --- please make sure
that they are adequately backed up, though.

## Academic Honesty

Cheating on homework or the project will result in a score
of zero on the affected material, and will be reported to
appropriate PSU authorities.  Plagiarism is a form of
cheating. Please do not let me catch you plagiarizing.


> Plagiarism: n  
> 1. A piece of writing/work that has been copied from
> someone else and is presented as being your own
> work.  
> 2. the act of plagiarizing; taking someone's words or
> ideas as if they were your own.  
> —www.dictionary.com

If you use code, ideas, or text authored by someone else,
cite them. It is OK to get help from external sources of
knowledge, but citation is mandatory.

## Safe Space

PSU and your instructor are committed to providing a safe
and effective learning space for people of all races and
ethnicities, genders and gender roles, religious beliefs,
physical abilities, etc. Students in this course must
conduct themselves collegially and professionally.  Bigotry
or uncivility of any type will not be tolerated: this
behavior will result in removal from the course and
reporting to the appropriate authorities at PSU.

## Access and Inclusion for Students with Disabilities

PSU values diversity and inclusion; we are committed to
fostering mutual respect and full participation for all
students. My goal is to create a learning environment that
is equitable, useable, inclusive, and welcoming. If any
aspects of instruction or course design result in barriers
to your inclusion or learning, please notify me. The
Disability Resource Center (DRC) provides reasonable
accommodations for students who encounter barriers in the
learning environment.

If you have, or think you may have, a disability that may
affect your work in this class and feel you need
accommodations, contact the Disability Resource Center to
schedule an appointment and initiate a conversation about
reasonable accommodations. The DRC is located in 116 Smith
Memorial Student Union, 503-725-4150, <drc@pdx.edu>,
<https://www.pdx.edu/drc>.

If you already have accommodations, please contact me to
make sure that I have received a faculty notification letter
and discuss your accommodations. Students who need
accommodations for tests and quizzes are expected to
schedule their tests to overlap with the time the class is
taking the test.

For information about emergency preparedness, please go to
the Fire and Life Safety webpage
<https://www.pdx.edu/environmental-health-safety/fire-and-life-safety>
for information.

## Discrimination and Harrassment; Mandatory Reporting

Portland State is committed to providing an environment free
of all forms of prohibited discrimination and sexual
harassment (sexual assault, domestic and dating violence,
gender or sex-based harassment and stalking). If you have
experienced any form of gender or sex-based discrimination
or sexual harassment, know that help and support are
available. PSU has staff members trained to support
survivors in navigating campus life, accessing health and
counseling services, providing academic and on-housing
accommodations, helping with legal protective orders, and
more. Information about PSU's support services on campus,
including confidential services and reporting options, can
be found on PSU's Sexual Misconduct Prevention and Response
website at <http://www.pdx.edu/sexual-assault/get-help>. You
may call a Confidential Advocate at 503-725-5672 or by
scheduling on-line: <http://psuwrc.youcanbook.me>. You may
also report any incident of discrimination or discriminatory
harassment, including sexual harassment, to either the
[Office of Equity and Compliance](http://www.pdx.edu/diversity/office-of-equity-compliance)
or the
[Office of the Dean of Student Life](http://www.pdx.edu/dos/student-conduct-at-psu).

Please be aware that all PSU faculty members and instructors
are required to report information of an incident that may
constitute prohibited discrimination, including sexual
harassment and sexual violence. This means that if you tell
me about a situation of sexual harassment or sexual violence
that may have violated university policy or student code of
conduct, I have to share the information with my supervisor,
the University's Title IX Coordinator or the Office of the
Dean of Student Life. For more information about these
matters that include Title IX, please complete the required
student module
[Creating a Safe Campus](http://www.pdx.edu/sexual-assault/safe-campus-module)
in your D2L.
