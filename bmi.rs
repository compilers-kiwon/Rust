// BMI 판정용 구조체
struct BmiRange {
    min: f64,       // min 이상
    max: f64,       // max 미만
    label: String,  // 판정
}

fn main() {
    // 키와 몸무게 입력
    let height_cm = input("키(cm): ");
    let weight = input("몸무게(kg): ");
    
    // BMI 계산
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("BMI={:.1}", bmi);

    // 비만도 판정표를 벡터 타입으로 생성
    let bmi_table = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "저체중".to_string(),
        },
        BmiRange {
            min: 18.5,
            max: 23.0,
            label: "정상".to_string(),
        },
        BmiRange {
            min: 23.0,
            max: 25.0,
            label: "비만전단계".to_string(),
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "1단계 비만".to_string(),
        },
        BmiRange {
            min: 30.0,
            max: 35.0,
            label: "2단계 비만".to_string(),
        },
        BmiRange {
            min: 35.0,
            max: 99.0,
            label: "3단계 비만".to_string(),
        },
    ];

    // 비만도 진단
    let mut result = "판단 불가".to_string();

    for range in bmi_table {
        if range.min<=bmi && bmi<range.max {
            result = range.label;
            break;
        }
    }

    // 결과 표시
    println!("비만도 = {}",result);
}

fn input(prompt: &str) -> f64 {
    // 메세지 출력
    println!("{}", prompt);
    // 입력값을 가져옴
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("잘못된 입력입니다.");
    // 공백을 제거하고 숫자 값으로 변환
    return  s.trim().parse().expect("숫자가 아닙니다.");
}