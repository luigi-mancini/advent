const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const Offset = struct { y: i32, x: i32 };
const offsets: [8]Offset = .{
    .{ .y = -1, .x = -1 },
    .{ .y = -1, .x = 0 },
    .{ .y = -1, .x = 1 },
    .{ .y = 0, .x = -1 },
    .{ .y = 0, .x = 1 },
    .{ .y = 1, .x = -1 },
    .{ .y = 1, .x = 0 },
    .{ .y = 1, .x = 1 },
};

const Coords = struct { y: usize, x: usize };

pub fn checkRollAtPositionAndOffset(list: []const []u8, y: usize, x: usize, off_y: i32, off_x: i32) bool {
    const y_cast: i32 = @intCast(y);
    const x_cast: i32 = @intCast(x);

    const y_new = y_cast + off_y;
    const x_new = x_cast + off_x;

    if (y_new < 0 or x_new < 0 or y_new >= list.len or x_new >= list[0].len) {
        return false;
    }

    const y_indx: usize = @intCast(y_new);
    const x_indx: usize = @intCast(x_new);
    return list[y_indx][x_indx] == '@';
}

pub fn findRollsToRemove(list: []const []u8, coords: *ArrayList(Coords)) !u64 {
    var rolls: u32 = 0;
    for (0..list.len) |y| {
        for (0..list[0].len) |x| {
            if (list[y][x] != '@') {
                continue;
            }

            var count: u32 = 0;
            for (offsets) |o| {
                if (checkRollAtPositionAndOffset(list, y, x, o.y, o.x)) {
                    count += 1;
                    if (count >= 4) {
                        break;
                    }
                }
            }

            if (count < 4) {
                try coords.*.append(allocator, .{ .y = y, .x = x });
                rolls += 1;
            }
        }
    }
    return rolls;
}

pub fn checkRolls(list: *ArrayList([]u8), remove: bool) !u64 {
    var coords = try ArrayList(Coords).initCapacity(allocator, 500);
    defer coords.deinit(allocator);

    var total_count: u64 = 0;

    while (true) {
        const count = try findRollsToRemove(list.*.items, &coords);
        total_count += count;

        if (!remove or count == 0)
            break;

        for (coords.items) |c| {
            list.*.items[c.y][c.x] = '.';
        }
    }
    return total_count;
}

pub fn readInput() !ArrayList([]u8) {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day4.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    var list = try ArrayList([]u8).initCapacity(allocator, 500);

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

    return list;
}

pub fn main() !void {
    defer _ = gpa.deinit();

    var list = try readInput();
    defer {
        std.debug.print("Dealloc", .{});
        for (list.items) |l| {
            allocator.free(l);
        }
        list.deinit(allocator);
    }

    //for (list.items) |l| {
    //    std.debug.print("{s}\n", .{l});
    //}

    const rolls = try checkRolls(&list, false);
    std.debug.print("We can access {d} rolls.\n", .{rolls});

    const rolls2 = try checkRolls(&list, true);
    std.debug.print("Part 2: We can access {d} rolls.\n", .{rolls2});
}
