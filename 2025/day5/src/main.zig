const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const Range = struct { start: u64, end: u64 };

pub fn countFreshIngredients(ranges: []Range, ids: []u64) u64 {
    var count: u64 = 0;
    for (ids) |i| {
        for (ranges) |r| {
            if (i >= r.start and i <= r.end) {
                count += 1;
                break;
            }
        }
    }
    return count;
}

pub fn readInput(range: *ArrayList(Range), ids: *ArrayList(u64)) !void {
    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day5.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    while (reader.takeDelimiterExclusive('\n')) |line| {
        const str = std.mem.trimRight(u8, line, "\r\n");

        if (std.mem.indexOfScalar(u8, str, '-')) |index| {
            const start = try std.fmt.parseInt(u64, str[0..index], 10);
            const end = try std.fmt.parseInt(u64, str[index + 1 ..], 10);

            try range.append(allocator, .{ .start = start, .end = end });
        } else {
            if (str.len != 0) {
                const id = try std.fmt.parseInt(u64, str, 10);
                try ids.append(allocator, id);
            }
        }
        _ = try reader.discardDelimiterInclusive('\n');
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.StreamTooLong, // line could not fit in buffer
        error.ReadFailed, // caller can check reader implementation for diagnostics
        => |e| return e,
    }
}

pub fn main() !void {
    defer _ = gpa.deinit();

    var ranges = try ArrayList(Range).initCapacity(allocator, 500);
    var ids = try ArrayList(u64).initCapacity(allocator, 500);
    defer ranges.deinit(allocator);
    defer ids.deinit(allocator);

    try readInput(&ranges, &ids);
    //defer {
    //    std.debug.print("Dealloc", .{});
    //    for (list.items) |l| {
    //        allocator.free(l);
    //    }
    //    list.deinit(allocator);
    //}

    for (ranges.items) |r| {
        std.debug.print("{}\n", .{r});
    }

    for (ids.items) |i| {
        std.debug.print("{d}\n", .{i});
    }

    const count = countFreshIngredients(ranges.items, ids.items);
    std.debug.print("We have {d} fresh ingredients.\n", .{count});

    //const rolls2 = try checkRolls(&list, true);
    //std.debug.print("Part 2: We can access {d} rolls.\n", .{rolls2});
}
