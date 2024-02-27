use tomekeeper::modules::tree;
 

fn main() {
    let work_dir = "workdir";
    // println!("pwd: \n{}", pwd().display());
    // println!("=====================");
    // println!("ls: ");
    // ls(None);
    // println!("ls {}:", work_dir);
    // ls(Some(work_dir));
    // println!("=====================");
    // println!("metadata: None");
    // metadata(None);
    // println!("metadata: workdir/d1");
    // metadata(Some("workdir/d1/"));
    // println!("metadata: workdir/d1/f1");
    // metadata(Some("workdir/d1/f1"));

    let mut treee = tree::Tree::new(Some(work_dir));
    // let mut treee = tree::Tree::new(None);
    // println!("{:?}", treee.pwd());
    treee.build_tree();
    println!("{}", treee.to_json());
}
