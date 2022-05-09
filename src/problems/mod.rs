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
pub mod problem101;
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

pub const PROBLEMS: [fn() -> String; 101] = [
    problem01::solve,
    problem02::solve,
    problem03::solve,
    problem04::solve,
    problem05::solve,
    problem06::solve,
    problem07::solve,
    problem08::solve,
    problem09::solve,
    problem10::solve,
    problem11::solve,
    problem12::solve,
    problem13::solve,
    problem14::solve,
    problem15::solve,
    problem16::solve,
    problem17::solve,
    problem18::solve,
    problem19::solve,
    problem20::solve,
    problem21::solve,
    problem22::solve,
    problem23::solve,
    problem24::solve,
    problem25::solve,
    problem26::solve,
    problem27::solve,
    problem28::solve,
    problem29::solve,
    problem30::solve,
    problem31::solve,
    problem32::solve,
    problem33::solve,
    problem34::solve,
    problem35::solve,
    problem36::solve,
    problem37::solve,
    problem38::solve,
    problem39::solve,
    problem40::solve,
    problem41::solve,
    problem42::solve,
    problem43::solve,
    problem44::solve,
    problem45::solve,
    problem46::solve,
    problem47::solve,
    problem48::solve,
    problem49::solve,
    problem50::solve,
    problem51::solve,
    problem52::solve,
    problem53::solve,
    problem54::solve,
    problem55::solve,
    problem56::solve,
    problem57::solve,
    problem58::solve,
    problem59::solve,
    problem60::solve,
    problem61::solve,
    problem62::solve,
    problem63::solve,
    problem64::solve,
    problem65::solve,
    problem66::solve,
    problem67::solve,
    problem68::solve,
    problem69::solve,
    problem70::solve,
    problem71::solve,
    problem72::solve,
    problem73::solve,
    problem74::solve,
    problem75::solve,
    problem76::solve,
    problem77::solve,
    problem78::solve,
    problem79::solve,
    problem80::solve,
    problem81::solve,
    problem82::solve,
    problem83::solve,
    problem84::solve,
    problem85::solve,
    problem86::solve,
    problem87::solve,
    problem88::solve,
    problem89::solve,
    problem90::solve,
    problem91::solve,
    problem92::solve,
    problem93::solve,
    problem94::solve,
    problem95::solve,
    problem96::solve,
    problem97::solve,
    problem98::solve,
    problem99::solve,
    problem100::solve,
    problem101::solve,
];

#[allow(dead_code)]
pub const SOLUTIONS: [&str; 101] = [
    "233168",
    "4613732",
    "6857",
    "906609",
    "232792560",
    "25164150",
    "104743",
    "23514624000",
    "31875000",
    "142913828922",
    "70600674",
    "76576500",
    "5537376230",
    "837799",
    "137846528820",
    "1366",
    "21124",
    "1074",
    "171",
    "648",
    "31626",
    "871198282",
    "4179871",
    "2783915460",
    "4782",
    "983",
    "-59231",
    "669171001",
    "9183",
    "443839",
    "73682",
    "45228",
    "100",
    "40730",
    "55",
    "872187",
    "748317",
    "932718654",
    "840",
    "210",
    "7652413",
    "162",
    "16695334890",
    "5482660",
    "1533776805",
    "5777",
    "134043",
    "9110846700",
    "296962999629",
    "997651",
    "121313",
    "142857",
    "4075",
    "376",
    "249",
    "972",
    "153",
    "26241",
    "129448",
    "26033",
    "28684",
    "127035954683",
    "49",
    "1322",
    "272",
    "661",
    "7273",
    "6531031914842725",
    "510510",
    "8319823",
    "428570",
    "303963552391",
    "7295372",
    "402",
    "161667",
    "190569291",
    "71",
    "55374",
    "73162890",
    "40886",
    "427337",
    "260324",
    "425185",
    "101524",
    "2772",
    "1818",
    "1097343",
    "7587457",
    "743",
    "1217",
    "14234",
    "8581146",
    "1258",
    "518408346",
    "14316",
    "24702",
    "8739992577",
    "18769",
    "709",
    "756872327473",
    "37076114526",
];

#[cfg(not(tarpaulin_include))]
pub fn run_all_problems() {
    for problem in PROBLEMS {
        problem();
    }
}
