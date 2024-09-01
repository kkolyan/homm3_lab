use homm3_lab_rs::tournament::arrange_tournament;

fn main() {
    arrange_tournament(1, include_str!("../../data/h3/CRTRAIT0.txt"), include_str!("../../data/h3/structure.txt"));
}