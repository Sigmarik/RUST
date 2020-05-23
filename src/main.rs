use text_io::read;

type point = (i32, i32);
type fig = (i32, i32, i32);

struct field {figs : Vec<fig>}
impl field {
    fn check(f : fig) -> Vec<point> {
        let mut answ : Vec<point> = Vec::new();
        let (typ, x, y) = f;
        if typ == 0 {
            for i in -10..10 {
                if i != 0 {
                    answ.push((x + i, y));
                    answ.push((x, y + i));
                    answ.push((x + i, y + i));
                    answ.push((x + i, y - i));
                }
            }
        } else if typ == 1 {
            for turn in vec![(-1, 2), (1, 2), (2, 1), (2, -1), (-1, -2), (1, -2), (-2, 1), (-2, -1)] {
                let (dx, dy) = turn;
                answ.push((x + dx, y + dy));
            }
        } else if typ == 2 {
            for i in -10..10 {
                if i != 0 {
                    answ.push((x + i, y));
                    answ.push((x, y + i));
                }
            }
        } else if typ == 3 {
            for i in -10..10 {
                if i != 0 {
                    answ.push((x + i, y + i));
                    answ.push((x + i, y - i));
                }
            }
        }
        answ
    }
    fn add(&mut self, typ : i32, x : i32, y : i32) -> bool {
        let mut OK : bool = true;
        let res : Vec<point> = field::check((typ, x, y));
        for f in 0..self.figs.len() {
            for p in 0..res.len() {
                let (xx, yy) = *res.get(p).unwrap();
                let (t, X, Y) = *self.figs.get(f).unwrap();
                if xx == X && yy == Y {
                    OK = false;
                }
            }
        }
        if OK {
            self.figs.push((typ, x, y))
        }
        OK
    }
    fn ask(&mut self) {
        println!("Привязки типов:\n0-ферзь\n1-конь\n2-ладья\n3-слон\n");
        for i in 1..8 {
            for j in 1..8 {
                let mut OK : bool = true;
                for f in 0..self.figs.len() {
                    let (t, x, y) = *self.figs.get(f).unwrap();
                    if x == i && y == j {
                        print!(" {}", t);
                        OK = false;
                    }
                }
                if OK {
                    print!(" #")
                }
            }
            print!("\n");
        }
        println!("Введите запрос на добавление фигуры вида (тип X Y)");
        let tp : i32 = read!();
        let xx : i32 = read!();
        let yy : i32 = read!();
        let correct : bool = self.add(tp, xx, yy);
        if correct {
            println!("Корректно!")
        } else {
            println!("Некорректно!")
        }
    }
}

fn main() {
    let mut fld : field = field{ figs: vec![] };
    while true {
        fld.ask();
    }
}
