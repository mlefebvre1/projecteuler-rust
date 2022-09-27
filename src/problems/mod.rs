pub mod problem01;
pub mod problem02;
pub mod problem03;
pub mod problem04;
pub mod problem05;
pub mod problem06;
pub mod problem07;
pub mod problem08;
pub mod problem09;
pub mod problem10;
pub mod problem100;
pub mod problem11;
pub mod problem12;
pub mod problem13;
pub mod problem14;
pub mod problem15;
pub mod problem16;
pub mod problem17;
pub mod problem18;
pub mod problem19;
pub mod problem20;
pub mod problem21;
pub mod problem22;
pub mod problem23;
pub mod problem24;
pub mod problem25;
pub mod problem26;
pub mod problem27;
pub mod problem28;
pub mod problem29;
pub mod problem30;
pub mod problem31;
pub mod problem32;
pub mod problem33;
pub mod problem34;
pub mod problem35;
pub mod problem36;
pub mod problem37;
pub mod problem38;
pub mod problem39;
pub mod problem40;
pub mod problem41;
pub mod problem42;
pub mod problem43;
pub mod problem44;
pub mod problem45;
pub mod problem46;
pub mod problem47;
pub mod problem48;
pub mod problem49;
pub mod problem50;
pub mod problem51;
pub mod problem52;
pub mod problem53;
pub mod problem54;
pub mod problem55;
pub mod problem56;
pub mod problem57;
pub mod problem58;
pub mod problem59;
pub mod problem60;
pub mod problem61;
pub mod problem62;
pub mod problem63;
pub mod problem64;
pub mod problem65;
pub mod problem66;
pub mod problem67;
pub mod problem68;
pub mod problem69;
pub mod problem70;
pub mod problem71;
pub mod problem72;
pub mod problem73;
pub mod problem74;
pub mod problem75;
pub mod problem76;
pub mod problem77;
pub mod problem78;
pub mod problem79;
pub mod problem80;
pub mod problem81;
pub mod problem82;
pub mod problem83;
pub mod problem84;
pub mod problem85;
pub mod problem86;
pub mod problem87;
pub mod problem88;
pub mod problem89;
pub mod problem90;
pub mod problem91;
pub mod problem92;
pub mod problem93;
pub mod problem94;
pub mod problem95;
pub mod problem96;
pub mod problem97;
pub mod problem98;
pub mod problem99;

use lazy_static::lazy_static;

pub struct Problem(pub fn() -> String, pub &'static str);

lazy_static! {
    pub static ref PROBLEMS: Vec<Problem> = vec![
        Problem(problem01::solve, "233168"),
        Problem(problem02::solve, "4613732"),
        Problem(problem03::solve, "6857"),
        Problem(problem04::solve, "906609"),
        Problem(problem05::solve, "232792560"),
        Problem(problem06::solve, "25164150"),
        Problem(problem07::solve, "104743"),
        Problem(problem08::solve, "23514624000"),
        Problem(problem09::solve, "31875000"),
        Problem(problem10::solve, "142913828922"),
        Problem(problem11::solve, "70600674"),
        Problem(problem12::solve, "76576500"),
        Problem(problem13::solve, "5537376230"),
        Problem(problem14::solve, "837799"),
        Problem(problem15::solve, "137846528820"),
        Problem(problem16::solve, "1366"),
        Problem(problem17::solve, "21124"),
        Problem(problem18::solve, "1074"),
        Problem(problem19::solve, "171"),
        Problem(problem20::solve, "648"),
        Problem(problem21::solve, "31626"),
        Problem(problem22::solve, "871198282"),
        Problem(problem23::solve, "4179871"),
        Problem(problem24::solve, "2783915460"),
        Problem(problem25::solve, "4782"),
        Problem(problem26::solve, "983"),
        Problem(problem27::solve, "-59231"),
        Problem(problem28::solve, "669171001"),
        Problem(problem29::solve, "9183"),
        Problem(problem30::solve, "443839"),
        Problem(problem31::solve, "73682"),
        Problem(problem32::solve, "45228"),
        Problem(problem33::solve, "100"),
        Problem(problem34::solve, "40730"),
        Problem(problem35::solve, "55"),
        Problem(problem36::solve, "872187"),
        Problem(problem37::solve, "748317"),
        Problem(problem38::solve, "932718654"),
        Problem(problem39::solve, "840"),
        Problem(problem40::solve, "210"),
        Problem(problem41::solve, "7652413"),
        Problem(problem42::solve, "162"),
        Problem(problem43::solve, "16695334890"),
        Problem(problem44::solve, "5482660"),
        Problem(problem45::solve, "1533776805"),
        Problem(problem46::solve, "5777"),
        Problem(problem47::solve, "134043"),
        Problem(problem48::solve, "9110846700"),
        Problem(problem49::solve, "296962999629"),
        Problem(problem50::solve, "997651"),
        Problem(problem51::solve, "121313"),
        Problem(problem52::solve, "142857"),
        Problem(problem53::solve, "4075"),
        Problem(problem54::solve, "376"),
        Problem(problem55::solve, "249"),
        Problem(problem56::solve, "972"),
        Problem(problem57::solve, "153"),
        Problem(problem58::solve, "26241"),
        Problem(problem59::solve, "129448"),
        Problem(problem60::solve, "26033"),
        Problem(problem61::solve, "28684"),
        Problem(problem62::solve, "127035954683"),
        Problem(problem63::solve, "49"),
        Problem(problem64::solve, "1322"),
        Problem(problem65::solve, "272"),
        Problem(problem66::solve, "661"),
        Problem(problem67::solve, "7273"),
        Problem(problem68::solve, "6531031914842725"),
        Problem(problem69::solve, "510510"),
        Problem(problem70::solve, "8319823"),
        Problem(problem71::solve, "428570"),
        Problem(problem72::solve, "303963552391"),
        Problem(problem73::solve, "7295372"),
        Problem(problem74::solve, "402"),
        Problem(problem75::solve, "161667"),
        Problem(problem76::solve, "190569291"),
        Problem(problem77::solve, "71"),
        Problem(problem78::solve, "55374"),
        Problem(problem79::solve, "73162890"),
        Problem(problem80::solve, "40886"),
        Problem(problem81::solve, "427337"),
        Problem(problem82::solve, "260324"),
        Problem(problem83::solve, "425185"),
        Problem(problem84::solve, "101524"),
        Problem(problem85::solve, "2772"),
        Problem(problem86::solve, "1818"),
        Problem(problem87::solve, "1097343"),
        Problem(problem88::solve, "7587457"),
        Problem(problem89::solve, "743"),
        Problem(problem90::solve, "1217"),
        Problem(problem91::solve, "14234"),
        Problem(problem92::solve, "8581146"),
        Problem(problem93::solve, "1258"),
        Problem(problem94::solve, "518408346"),
        Problem(problem95::solve, "14316"),
        Problem(problem96::solve, "24702"),
        Problem(problem97::solve, "8739992577"),
        Problem(problem98::solve, "18769"),
        Problem(problem99::solve, "709"),
        Problem(problem100::solve, "756872327473")
    ];
}

#[cfg(not(tarpaulin_include))]
pub fn run_all_problems() {
    for Problem(solve, _) in PROBLEMS.iter() {
        solve();
    }
}
