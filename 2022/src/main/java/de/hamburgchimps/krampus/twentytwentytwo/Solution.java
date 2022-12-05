package de.hamburgchimps.krampus.twentytwentytwo;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public final class Solution {
    private static final List<List<Supplier<Result>>> store = List.of(DayOne.store);

    public static Result execute(int day, int part) {
        return store
                .get(day - 1)
                .get(part - 1)
                .get();
    }

    public static final class DayOne {
        private static final List<String> input = getInput("day1.txt");
        private static final List<Supplier<Result>> store = List.of(DayOne::PartOne, DayOne::PartTwo);

        private static Result PartOne() {
            var inventories = new ArrayList<List<Integer>>();
            var currentInventoryIdx = 0;

            inventories.add(currentInventoryIdx, new ArrayList<>());

            for (String line: input) {
                if (line.isEmpty()) {
                    inventories.add(++currentInventoryIdx, new ArrayList<>());
                    continue;
                }

                inventories
                        .get(currentInventoryIdx)
                        .add(Integer.parseInt(line));
            }

            return new Result(input, inventories, true);
        }

        private static Result PartTwo() {
            return new Result(input, List.of(),true);
        }
    }

    public record Result(List<String> input, List<List<Integer>> output, boolean success) {
    }

    // Would not have figured this out without https://stackoverflow.com/a/46613809/205930
    // Who knew reading files in the resource folder was so complicated?
    private static List<String> getInput(String path) {
        try (var stream = Solution.class.getClassLoader().getResourceAsStream("input" + "/" + path)) {
            if (stream == null) {
                return List.of();
            }
            var reader = new InputStreamReader(stream);
            var br = new BufferedReader(reader);

            return br.lines()
                    .toList();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }
}
