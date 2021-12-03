const std = @import("std");

const len: comptime usize = 12;

pub fn main() anyerror!void {
    const stdin = std.io.getStdIn().reader();

    var buf: [len + 1]u8 = undefined;

    var ones = [_]i32{0} ** len;
    var total: i32 = 0;
    while (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        for (buf[0 .. buf.len - 1]) |chr, i| {
            if (chr == '1') {
                ones[i] += 1;
            }
        }
        total += 1;
    }

    var gamma = [_]u8{0} ** len;
    var epsilon = [_]u8{0} ** len;

    const median = @divTrunc(total, 2);
    for (ones) |count, i| {
        if (count > median) {
            gamma[i] = '1';
            epsilon[i] = '0';
        } else {
            gamma[i] = '0';
            epsilon[i] = '1';
        }
    }

    const gammaDec = bin2dec(gamma);
    const epsilonDec = bin2dec(epsilon);

    std.debug.print("{s} {d} {s} {d}: {d}\n", .{ gamma, gammaDec, epsilon, epsilonDec, gammaDec * epsilonDec });
}

fn bin2dec(bin: [len]u8) u32 {
    var sum: u32 = 0;
    for (bin) |c, i| {
        const shift: u5 = (@intCast(u5, len - 1) - @intCast(u5, i));
        sum |= @shlExact(@as(u32, @boolToInt(c == '1')), shift);
    }
    return sum;
}
