use rand::Rng;

// 전체 미로 크기 지정
const MAP_N: usize = 25;

// #define
const EMPTY: usize = 0;
const WALL: usize = 1;

fn main() {
    // 난수 생성기 준비
    let mut rng = rand::thread_rng();

    // 미로 초기화
    let mut maze = [[EMPTY;MAP_N];MAP_N];

    // 둘레를 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = WALL;
        maze[0][n] = WALL;
        maze[n][MAP_N-1] = WALL;
        maze[MAP_N-1][n] = WALL;
    }

    let is_odd = |n| n%2 == 1;

    // 2칸마다 1개의 벽을 설치
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if is_odd(x) || is_odd(y) {continue;}
            
            // 벽 설치
            maze[y][x] = WALL;

            // 상하좌우 중 하나에 벽을 또 설치
            let r = rng.gen_range(0..=3);

            // 0:상, 1:하, 2:좌, 3:우
            match r {
                0 => maze[y-1][x] = WALL,
                1 => maze[y+1][x] = WALL,
                2 => maze[y][x-1] = WALL,
                3 => maze[y][x+1] = WALL,
                _ => {},
            }
        }
    }

    // 입구 & 출구
    maze[1][0] = EMPTY;
    maze[MAP_N-2][MAP_N-1] = EMPTY;
    
    // 미로 출력
    // 0:EMPTY=>' ', 1:WALL=>'#'
    let tiles = [" ","#"];

    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}",tiles[maze[y][x]]);
        }
        println!("");
    }
}