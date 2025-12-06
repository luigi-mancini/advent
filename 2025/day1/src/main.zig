const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const Direction = enum { L, R };

const Move = struct { direction: Direction, ticks: u32 };

pub fn createMove(str: []u8) !Move {
    const d = switch (str[0]) {
        'L' => Direction.L,
        'R' => Direction.R,
        else => {
            @panic("Unexpected direction!");
        },
    };

    const x = try std.fmt.parseInt(u32, str[1..], 10);

    return .{ .direction = d, .ticks = x };
}

pub fn processMoves(moves: []const Move) u32 {
    var current_val: u32 = 50;
    var num_zeroes: u32 = 0;

    for (moves) |item| {
        const t = item.ticks % 100;

        current_val =
            switch (item.direction) {
                .L => if (t > current_val)
                    100 - (t - current_val)
                else
                    current_val - t,
                .R => (current_val + t) % 100,
            };

        if (current_val == 0)
            num_zeroes += 1;
    }
    return num_zeroes;
}

pub fn processMoves_part2(moves: []const Move) u32 {
    var current_val: u32 = 50;
    var num_zeroes: u32 = 0;

    for (moves) |item| {
        num_zeroes += item.ticks / 100;
        const t = item.ticks % 100;

        const end_val =
            switch (item.direction) {
                .L => if (t > current_val)
                    100 - (t - current_val)
                else
                    current_val - t,
                .R => (current_val + t) % 100,
            };

        if (end_val == 0) {
            num_zeroes += 1;
        } else if (current_val != 0 and ((item.direction == .L and end_val > current_val) or (item.direction == .R and end_val < current_val))) {
            num_zeroes += 1;
        }
        current_val = end_val;
    }
    return num_zeroes;
}

pub fn main() !void {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day1a.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    var list = try ArrayList(Move).initCapacity(allocator, 500);
    defer list.deinit(allocator);

    while (reader.takeDelimiterExclusive('\n')) |line| {
        _ = try reader.discardDelimiterInclusive('\n');
        const m = try createMove(line);
        try list.append(allocator, m);
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.StreamTooLong, // line could not fit in buffer
        error.ReadFailed, // caller can check reader implementation for diagnostics
        => |e| return e,
    }

    // for (list.items) |move| {
    //    std.debug.print("{d} {d}\n", .{ move.direction, move.ticks });
    //}

    const password = processMoves(list.items);
    std.debug.print("The password is {d}\n", .{password});

    const password2 = processMoves_part2(list.items);
    std.debug.print("The new password is {d}\n", .{password2});
}
