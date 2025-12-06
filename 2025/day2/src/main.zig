const std = @import("std");
const ArrayList = std.ArrayList;

var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa.allocator();

const MyErrors = error{
    ParseError,
};

const Range = struct { start: u64, end: u64 };

pub fn createRange(str: []const u8) !Range {
    if (std.mem.indexOfScalar(u8, str, '-')) |index| {
        const start = try std.fmt.parseInt(u64, str[0..index], 10);
        const end = try std.fmt.parseInt(u64, str[index + 1 ..], 10);
        return .{ .start = start, .end = end };
    } else {
        return MyErrors.ParseError;
    }
}

pub fn is_valid_key(key: []u8) bool {
    if (key.len % 2 != 0) {
        return false;
    }

    const midpoint = key.len / 2;
    return std.mem.eql(u8, key[0..midpoint], key[midpoint..]);
}

pub fn is_valid_key_part2(key: []u8) bool {
    const midpoint = key.len / 2;

    for (1..midpoint + 1) |pattern_len| {
        if (key.len % pattern_len != 0) {
            continue;
        }

        const repeats = key.len / pattern_len - 1;
        const pattern = key[0..pattern_len];

        var start = pattern_len;
        var end = 2 * pattern_len;

        var match_found = true;
        var i: u8 = 0;
        while (i < repeats) : (i += 1) {
            if (!std.mem.eql(u8, pattern, key[start..end])) {
                match_found = false;
                break;
            }
            start += pattern_len;
            end += pattern_len;
        }

        if (match_found) {
            return true;
        }
    }

    return false;
}

const ValidKeyCheck = *const fn ([]u8) bool;

pub fn count_invalid_keys(ranges: []const Range, isValid: ValidKeyCheck) !u128 {
    var count: u128 = 0;

    var buffer: [1024]u8 = undefined;

    for (ranges) |r| {
        for (r.start..r.end + 1) |key| {
            const key_as_string = try std.fmt.bufPrint(&buffer, "{}", .{key});
            if (isValid(key_as_string)) {
                count += key;
            }
        }
    }
    return count;
}

pub fn main() !void {
    defer _ = gpa.deinit();

    const cwd = std.fs.cwd();
    const file = try cwd.openFile("day2.txt", .{ .mode = .read_only });
    defer file.close();

    var read_buffer: [1024]u8 = undefined;
    var fr = file.reader(&read_buffer);
    var reader = &fr.interface;

    var list = try ArrayList(Range).initCapacity(allocator, 500);
    defer list.deinit(allocator);

    while (reader.takeDelimiterExclusive(',')) |line| {
        const trimmed_line = std.mem.trimRight(u8, line, "\r\n");

        if (trimmed_line.len != 0) {
            const r = try createRange(trimmed_line);
            try list.append(allocator, r);
        }

        _ = reader.discardDelimiterInclusive(',') catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };
    } else |err| switch (err) {
        error.EndOfStream => {},
        error.StreamTooLong, // line could not fit in buffer
        error.ReadFailed, // caller can check reader implementation for diagnostics
        => |e| return e,
    }

    const invalid_keys = try count_invalid_keys(list.items, &is_valid_key);
    std.debug.print("Invalid key count is {d}\n", .{invalid_keys});

    const invalid_keys_part2 = try count_invalid_keys(list.items, &is_valid_key_part2);
    std.debug.print("Invalid key count for part 2 is {d}\n", .{invalid_keys_part2});
}
