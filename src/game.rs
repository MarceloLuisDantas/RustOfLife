use rand::prelude::*;

pub struct Board {
    pub board: Vec<Vec<char>>,
    last_generation: Vec<Vec<char>>
}

impl Board {
    fn vizinhos_vivos(&self, x: usize, y: usize) -> i8 {
        let mut vizinhos_vivos = 0;
        let vizinhos = vec![
            self.board[x-1][y-1],
            self.board[x-1][y],
            self.board[x-1][y+1],
            self.board[x][y-1],
            self.board[x][y+1],
            self.board[x+1][y-1],
            self.board[x+1][y],
            self.board[x+1][y+1],
        ];
        for i in vizinhos.iter() {
            if *i == '#' {
                vizinhos_vivos += 1;
            }
        }
        return vizinhos_vivos;
    }

    fn alterar_estado(&mut self, x: usize, y: usize) {
        if self.board[x][y] == '*' {
            self.board[x][y] = '#';
        } else if self.board[x][y] == '#' {
            self.board[x][y] = '*';
        }
    }

    pub fn voltar_geracao(&mut self) {
        self.board = self.last_generation.clone();
    }

    pub fn next_generation(&mut self, nascer: Vec<i8>, viver: Vec<i8>) {
        self.last_generation = self.board.clone();
        let mut matar_nascer: Vec<(usize, usize)> = Vec::new();
        for (x, linha) in self.board.iter().enumerate() {
            for (y, celula) in linha.iter().enumerate() {
                if *celula == '*' {
                    if nascer.contains(&self.vizinhos_vivos(x, y)) {
                        matar_nascer.push((x, y));
                    }
                } else if *celula == '#' {
                    if !viver.contains(&self.vizinhos_vivos(x, y)) {
                        matar_nascer.push((x, y));
                    }
                }
            }
        }
        for (x, y) in matar_nascer.iter() {
            self.alterar_estado(*x, *y);
        }
    }
}

pub fn cria_board(x: i32, y: i32) -> Board {
    let mut campo: Vec<Vec<char>> = Vec::new();
    let mut topo_base: Vec<char> = Vec::new();

    topo_base.push('l');
    for _ in 1..y {
        topo_base.push('-');
    }
    topo_base.push('r');
    
    campo.push(topo_base.clone());
    let mut gerador = rand::thread_rng();
    for _ in 1..x {
        let mut linha: Vec<char> = Vec::new();
        
        linha.push('l');
        for _ in 1..y {
            if gerador.gen_range(0..10) == 1 {
                linha.push('#');
            } else {
                linha.push('*');
            }
        }
        linha.push('r');

        campo.push(linha);
    }
    campo.push(topo_base);

    Board {
        board: campo.clone(),
        last_generation: campo,        
    }
}