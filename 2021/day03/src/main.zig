const std = @import("std");
const ArrayList = std.ArrayList;

const itemLen: comptime usize = 12;
const itemLenP1: comptime usize = itemLen + 1;

pub fn main() anyerror!void {
    var allocator = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer allocator.deinit();

    const stdin = std.io.getStdIn().reader();

    var numbers = ArrayList([itemLen]u8).init(&allocator.allocator);
    defer numbers.deinit();

    var buf: [itemLenP1]u8 = undefined;

    var ones = [_]i32{0} ** itemLen;
    var total: i32 = 0;
    while (try stdin.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        addOnes(buf[0 .. buf.len - 1], &ones);
        @memcpy(try numbers.addOne(), buf[0..itemLen], itemLen);
        total += 1;
    }

    var gamma = [_]u8{0} ** itemLen;
    var epsilon = [_]u8{0} ** itemLen;

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

    std.debug.print("poggies {s} {d} {s} {d}: {d}\n", .{ gamma, gammaDec, epsilon, epsilonDec, gammaDec * epsilonDec });

    // part 2

    var o2numbers = ArrayList([itemLen]u8).init(&allocator.allocator);
    defer o2numbers.deinit();
    var co2numbers = ArrayList([itemLen]u8).init(&allocator.allocator);
    defer co2numbers.deinit();

    try o2numbers.appendSlice(numbers.items);
    try co2numbers.appendSlice(numbers.items);

    inline for ([_]i32{0} ** itemLen) |_, i| {
        if (o2numbers.items.len > 1) {
            filter(&o2numbers, true, i, o2filter);
        }
        if (co2numbers.items.len > 1) {
            filter(&co2numbers, false, i, o2filter);
        }
    }

    const o2 = bin2dec(o2numbers.items[0]);
    const co2 = bin2dec(co2numbers.items[0]);
    std.debug.print("monaks {s} {s} = {d} {d}: {d}\n", .{ o2numbers.items[0], co2numbers.items[0], o2, co2, o2 * co2 });
}

fn o2filter(comptime isO2: bool, comptime i: usize, pog: [itemLen]u8, onesInPosI: usize, numSamples: usize) bool {
    comptime var toKeep: u8 = '1';
    comptime var toDrop: u8 = '0';

    if (!isO2) {
        toKeep = '0';
        toDrop = '1';
    }

    if ((onesInPosI * 2 >= numSamples)) {
        return pog[i] == toKeep;
    } else {
        return pog[i] == toDrop;
    }
}

fn filter(lst: *ArrayList([itemLen]u8), comptime isO2: bool, comptime j: usize, comptime f: anytype) void {
    var ones: usize = 0;
    for (lst.items) |number| {
        if (number[j] == '1') {
            ones += 1;
        }
    }
    const numSamples = lst.items.len;

    var i: isize = @intCast(isize, lst.items.len) - 1;
    while (i >= 0) : (i -= 1) {
        const k = @intCast(usize, i);
        if (!f(isO2, j, lst.items[k], ones, numSamples)) {
            _ = lst.swapRemove(k);
        }
    }
}

fn bin2dec(bin: [itemLen]u8) u32 {
    var sum: u32 = 0;
    for (bin) |c, i| {
        const shift = itemLen - 1 - i;
        sum |= @shlExact(@as(u32, @boolToInt(c == '1')), @intCast(u5, shift));
    }
    return sum;
}

fn addOnes(str: *const [itemLen]u8, stats: *[itemLen]i32) void {
    for (str) |c, i| {
        if (c == '1') {
            stats.*[i] += 1;
        }
    }
}
