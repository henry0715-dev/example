/*
    https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/
    학생의 위치와 의자의 거리 가장 이동이 적은 값 구하기
    1. seats, studentes 배열 정렬
    2. seats, studentes 배열에서 각 값의 차이 계산
    3. 값의 차이 계산 값에 대해서 절대값으로 도출

*/
pub fn test() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];

    println!(
        "min_moves_to_seat result : {}",
        min_moves_to_seat(seats, students)
    );
}
fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut sorted_seats = seats;
    let mut sorted_students = students;

    sorted_seats.sort_unstable();
    sorted_students.sort_unstable();

    sorted_seats
        .iter()
        .zip(sorted_students.iter())
        .map(|(seat, student)| (seat - student).abs())
        .sum()
}

#[test]
fn tc() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];

    let check = 4;
    let result = min_moves_to_seat(seats, students);

    assert_eq!(result, check);
}
