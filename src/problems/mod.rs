use anyhow::Result;
use lazy_static::lazy_static;

mod problem01;
mod problem02;
mod problem03;
mod problem04;
mod problem05;
mod problem06;
mod problem07;
mod problem08;
mod problem09;
mod problem10;
mod problem100;
mod problem11;
mod problem12;
mod problem13;
mod problem14;
mod problem15;
mod problem16;
mod problem17;
mod problem18;
mod problem19;
mod problem20;
mod problem21;
mod problem22;
mod problem23;
mod problem24;
mod problem25;
mod problem26;
mod problem27;
mod problem28;
mod problem29;
mod problem30;
mod problem31;
mod problem32;
mod problem33;
mod problem34;
mod problem35;
mod problem36;
mod problem37;
mod problem38;
mod problem39;
mod problem40;
mod problem41;
mod problem42;
mod problem43;
mod problem44;
mod problem45;
mod problem46;
mod problem47;
mod problem48;
mod problem49;
mod problem50;
mod problem51;
mod problem52;
mod problem53;
mod problem54;
mod problem55;
mod problem56;
mod problem57;
mod problem58;
mod problem59;
mod problem60;
mod problem61;
mod problem62;
mod problem63;
mod problem64;
mod problem65;
mod problem66;
mod problem67;
mod problem68;
mod problem69;
mod problem70;
mod problem71;
mod problem72;
mod problem73;
mod problem74;
mod problem75;
mod problem76;
mod problem77;
mod problem78;
mod problem79;
mod problem80;
mod problem81;
mod problem82;
mod problem83;
mod problem84;
mod problem85;
mod problem86;
mod problem87;
mod problem88;
mod problem89;
mod problem90;
mod problem91;
mod problem92;
mod problem93;
mod problem94;
mod problem95;
mod problem96;
mod problem97;
mod problem98;
mod problem99;

pub struct Problem(pub fn() -> Result<String>);

lazy_static! {
    pub static ref PROBLEMS: Vec<Problem> = vec![
        Problem(problem01::solve),
        Problem(problem02::solve),
        Problem(problem03::solve),
        Problem(problem04::solve),
        Problem(problem05::solve),
        Problem(problem06::solve),
        Problem(problem07::solve),
        Problem(problem08::solve),
        Problem(problem09::solve),
        Problem(problem10::solve),
        Problem(problem11::solve),
        Problem(problem12::solve),
        Problem(problem13::solve),
        Problem(problem14::solve),
        Problem(problem15::solve),
        Problem(problem16::solve),
        Problem(problem17::solve),
        Problem(problem18::solve),
        Problem(problem19::solve),
        Problem(problem20::solve),
        Problem(problem21::solve),
        Problem(problem22::solve),
        Problem(problem23::solve),
        Problem(problem24::solve),
        Problem(problem25::solve),
        Problem(problem26::solve),
        Problem(problem27::solve),
        Problem(problem28::solve),
        Problem(problem29::solve),
        Problem(problem30::solve),
        Problem(problem31::solve),
        Problem(problem32::solve),
        Problem(problem33::solve),
        Problem(problem34::solve),
        Problem(problem35::solve),
        Problem(problem36::solve),
        Problem(problem37::solve),
        Problem(problem38::solve),
        Problem(problem39::solve),
        Problem(problem40::solve),
        Problem(problem41::solve),
        Problem(problem42::solve),
        Problem(problem43::solve),
        Problem(problem44::solve),
        Problem(problem45::solve),
        Problem(problem46::solve),
        Problem(problem47::solve),
        Problem(problem48::solve),
        Problem(problem49::solve),
        Problem(problem50::solve),
        Problem(problem51::solve),
        Problem(problem52::solve),
        Problem(problem53::solve),
        Problem(problem54::solve),
        Problem(problem55::solve),
        Problem(problem56::solve),
        Problem(problem57::solve),
        Problem(problem58::solve),
        Problem(problem59::solve),
        Problem(problem60::solve),
        Problem(problem61::solve),
        Problem(problem62::solve),
        Problem(problem63::solve),
        Problem(problem64::solve),
        Problem(problem65::solve),
        Problem(problem66::solve),
        Problem(problem67::solve),
        Problem(problem68::solve),
        Problem(problem69::solve),
        Problem(problem70::solve),
        Problem(problem71::solve),
        Problem(problem72::solve),
        Problem(problem73::solve),
        Problem(problem74::solve),
        Problem(problem75::solve),
        Problem(problem76::solve),
        Problem(problem77::solve),
        Problem(problem78::solve),
        Problem(problem79::solve),
        Problem(problem80::solve),
        Problem(problem81::solve),
        Problem(problem82::solve),
        Problem(problem83::solve),
        Problem(problem84::solve),
        Problem(problem85::solve),
        Problem(problem86::solve),
        Problem(problem87::solve),
        Problem(problem88::solve),
        Problem(problem89::solve),
        Problem(problem90::solve),
        Problem(problem91::solve),
        Problem(problem92::solve),
        Problem(problem93::solve),
        Problem(problem94::solve),
        Problem(problem95::solve),
        Problem(problem96::solve),
        Problem(problem97::solve),
        Problem(problem98::solve),
        Problem(problem99::solve),
        Problem(problem100::solve)
    ];
}

#[cfg(not(tarpaulin_include))]
pub fn run_all_problems() -> Result<()> {
    for Problem(solve) in PROBLEMS.iter() {
        solve()?;
    }
    Ok(())
}
