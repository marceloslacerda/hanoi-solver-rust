use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pile_size: usize = match args.get(1) {
        Some(x) => x.parse().expect("Please provide an natural number"),
        None => 3
    };
    let mut piles: Vec<Vec<usize>> = (0..3).map(|_| vec![0; pile_size]).collect();
    for i in 0..pile_size {
        piles[0][i] = i+1;
    }
    let solution: Vec<usize> = (0..pile_size).map(|i| i+1).collect();
    print_towers(1, &piles);
    let sequence = {
        if pile_size % 2 == 0 { [(0,1), (0,2), (1,2)] }
        else { [(0,2), (0,1), (1,2)] }
    };
    let mut next: usize = 0;
    let mut step: usize = 1;
    while solution != piles[2] {
        legal_move(&mut piles, sequence[next].0, sequence[next].1);
        print_towers(step, &piles);
        next = (next+1) % 3;
        step += 1;
    }
}

fn legal_move(piles: &mut Vec<Vec<usize>>, t1: usize, t2: usize) {
    if is_legal(piles, t1, t2) {
        move_ring(piles, t1, t2);
    } else if is_legal(piles, t2, t1) {
        move_ring(piles, t2, t1);
    } else {
        panic!("No legal move between {:?} and {:?}", t1, t2);
    }
}

fn is_legal(piles: &Vec<Vec<usize>>, from_tower: usize, to_tower: usize) -> bool {
    if from_tower == to_tower {
        panic!("Origin and destination are the same");
    }
    let from_pile = piles.get(from_tower).expect("Invalid from_tower value");
    let ring_idx: usize = match from_pile.into_iter().position(| x: &usize| x!=&0) {
        Some(x) => x,
        None => {println!("Here"); return false}, // pile empty
    };
    let ring = from_pile[ring_idx];
    let to_pile = piles.get(to_tower).expect("Invalid to_tower value");
    let new_idx: usize = match to_pile.into_iter().rposition(| x: &usize| x==&0) {
        Some(x) => x,
        None => return false, // pile full
    };
    return new_idx == to_pile.len() - 1 || to_pile[new_idx+1] > ring;
}


fn move_ring(piles: &mut Vec<Vec<usize>>, from_tower: usize, to_tower: usize) {
    println!("Moving topmost ring from pile {} to pile {}", from_tower + 1, to_tower + 1);
    let from_pile = &mut piles[from_tower];
    let ring_idx: usize = from_pile.into_iter().position(| x: &mut usize| x!=&0).unwrap();
    let ring = from_pile[ring_idx];
    from_pile[ring_idx] = 0;
    let to_pile = &mut piles[to_tower];
    let new_idx = to_pile.into_iter().rposition(| x: &mut usize| x==&0).unwrap();
    to_pile[new_idx] = ring;
}

fn print_towers(turn: usize, piles: &Vec<Vec<usize>>) {
    println!("Towers turn {}:\n", turn);
    let mut buff = String::from("");
    for i in 0..piles[0].len() {
        buff.push_str("\n");
        for j in 0..3 {
            buff.push_str(&piles[j][i].to_string());
            if j != piles.len() -1 {
                buff.push_str("|");
            }
        }
    }
    buff.push_str("\n\n");
    println!("{}", buff);
}


