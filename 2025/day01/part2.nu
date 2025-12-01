def main [input: string] {
    let entries = open $input | parse --regex '^(?P<dir>[LR])(?P<ticks>\d+)$' | each {|row|
        ($row.ticks | into int ) * (if $row.dir == 'L' { -1 } else { 1 })
    }

    ($entries | reduce -f [0, 50] {|row, acc|
        let res = ($acc.1 + $row)
        mut count = $acc.0
        let diff = if $acc.1 == 0 { 0 } else 1
        if ($res <= 0) {
            $count += (($res | math abs ) // 100) + $diff
        } else if (100 <= $res) {
            $count += $res // 100
        }
        [$count, ($res mod 100)]
    }).0
}
