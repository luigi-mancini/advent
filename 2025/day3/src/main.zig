const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

pub fn getJoltage(bank: []u8, num_digits: u32) u64 {
    var max_val = allocator.alloc(u8, num_digits) catch |err| {
        std.debug.print("Error allocating memory {}\n", .{err});
        return 0;
    };
    @memset(max_val, 0);
    defer allocator.free(max_val);

    var start: usize = 0;
    for (0..num_digits) |indx| {
        const result = scanBankRange(start, num_digits - indx, bank);
        max_val[indx] = result.val;
        start = result.pos + 1;
    }

    // std.debug.print("Jolatage for {s} is {s}\n", .{ bank, max_val });

    return std.fmt.parseInt(u64, max_val, 10) catch |err| {
        std.debug.print("{}\n", .{err});
        return 0;
    };
}

pub fn scanBankRange(start: usize, digits_from_end: usize, bank: []u8) struct { val: u8, pos: usize } {
    const end: usize = bank.len - digits_from_end + 1;
    var max_val: u8 = 0;
    var max_pos: usize = 0;

    for (start..end) |indx| {
        if (bank[indx] > max_val) {
            max_val = bank[indx];
            max_pos = indx;
        }
    }

    return .{ .val = max_val, .pos = max_pos };
}

pub fn processBanks(banks: [][]u8, num_digits: u32) u64 {
    var joltage: u64 = 0;

    for (banks) |item| {
        joltage += getJoltage(item, num_digits);
    }
    return joltage;
}

pub fn main() !void {
    defer _ = gpa.deinit();

    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day3.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    var list = try ArrayList([]u8).initCapacity(allocator, 500);
    defer {
        for (list.items) |l| {
            allocator.free(l);
        }
        list.deinit(allocator);
    }

    while (reader.takeDelimiterExclusive('\n')) |line| {
        const l = try allocator.dupe(u8, line);
        try list.append(allocator, l);
        _ = try reader.discardDelimiterInclusive('\n');
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.StreamTooLong, // line could not fit in buffer
        error.ReadFailed, // caller can check reader implementation for diagnostics
        => |e| return e,
    }

    const joltage = processBanks(list.items, 2);
    std.debug.print("The total output joltage is {d}\n", .{joltage});

    const joltage_part2 = processBanks(list.items, 12);
    std.debug.print("The total output joltage for part 2 is {d}\n", .{joltage_part2});
}
