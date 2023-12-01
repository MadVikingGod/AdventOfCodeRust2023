if [ -z $1 ]; then
    echo Please supply a day number
    exit 1
fi
day="$(printf '%02d' $1)"

bin="src/bin/day_${day}.rs"
input="src/bin/input/day_${day}.txt"
touch ${bin} ${input}
if [ ! -s ${bin} ]; then
    cat >$bin <<EOF
fn main() {
    let input = include_str!("input/day_${day}.txt");
    println!("Hello, world!");
}

#[cfg(test)]
#[test]
fn test_nothing_day${day}() {
    ()
}
EOF
fi