// BMI 판정용 구조체
struct BmiRange {
    min: f64,       // min 이상
    max: f64,       // max 미만
    label: String,  // 판정
}

impl BmiRange {
    // 생성자
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {min, max, label: label.to_string(),}
    }
    // 범위 안에 있는지 확인하는 함수
    fn in_range(&self, v: f64) -> bool {
        (self.min <= v) && (v < self.max)
    }
}

// 학생의 정보(키,몸무게,이름)를 저장할 구조체
struct Body {
    height: f64,        // cm
    weight: Option<f64>,// kg
    name: String,       // 이름
}

impl Body {
    // 생성자
    fn new(height: f64, weight: Option<f64>, name: &str) -> Self {
        Body {height, weight, name: name.to_string(),}
    }
    // BMI 계산
    fn get_bmi(&self) -> Option<f64> {
        match self.weight {
            None => None,
            Some(w) => Some(w/(self.height/100.0).powf(2.0))
        }
    }
    // 비만 판정 출력
    fn print_result(&self, table: &Vec<BmiRange>) {
        // BMI 구하기
        let bmi: Option<f64> = self.get_bmi();
        let result = match bmi {
            None => "계산 불가".to_string(),
            Some(b) => {
                let mut ret_str = "".to_string();
                for range in table {
                    if range.in_range(b) {
                        ret_str = range.label.clone();
                        break;
                    } 
                }
                ret_str
            }
        };

        println!("{}님, BMI = {:.1}, 결과 = {}", self.name, 
                    match bmi { None=>0.0, Some(b)=>b }, result);
    }
}

fn main() {
    // 비만도 판정표를 벡터 타입으로 생성
    let bmi_table = vec![
        BmiRange::new(0.0,18.5,"저체중"),
        BmiRange::new(18.5,23.0,"정상"),
        BmiRange::new(23.0,25.0,"비만전단계"),
        BmiRange::new(25.0,30.0,"1단계 비만"),
        BmiRange::new(30.0,35.0,"2단계 비만"),
        BmiRange::new(35.0,99.0,"3단계 비만"),
    ];

    // 비만도를 진단할 학생들 정보
    let sung = Body::new(163.0, Some(75.2), "성은");
    let ga = Body::new(158.2, Some(55.0), "가빈");
    let chae = Body::new(174.2, None, "채연");

    // 비만도 진단
    sung.print_result(&bmi_table);
    ga.print_result(&bmi_table);
    chae.print_result(&bmi_table);
}