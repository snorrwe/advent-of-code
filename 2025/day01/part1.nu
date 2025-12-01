def main [input: string] {
    open $input 
    | parse --regex '^(?P<dir>[LR])(?P<ticks>\d+)$' 
    | each {|row|
        ($row.ticks | into int ) * (if $row.dir == 'L' { -1 } else { 1 })
    }
    | reduce -f [0, 50] {|row, acc|
        let res = (($acc.1 + $row) mod 100)
        mut count = $acc.0
        if $res == 0 { $count += 1 }
        [$count, $res]
    } 
    | get 0
}
