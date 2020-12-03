import functools


def main(number_of_entries):
    candidates = []

    with open("input") as f1:
        for index1, line1 in enumerate(f1):
            with open("input") as f2:
                for index2, line2 in enumerate(f2):
                    if index1 == index2:
                        continue

                    if len(candidates) == number_of_entries:
                        break

                    if int(line1) + int(line2) == 2020:
                        candidates.append(int(line1))
                        candidates.append(int(line2))

                if len(candidates) == number_of_entries:
                    break

    print(functools.reduce(lambda a, b: a * b, candidates))


if __name__ == "__main__":
    main(2)
    # main(3)
