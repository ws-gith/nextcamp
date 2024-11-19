from logging import INFO, basicConfig, getLogger

log = getLogger(__name__)
basicConfig(level=INFO)


def main() -> None:
    for i in range(1, 20):
        print(f"{i}: We back in python bitch!")
    log.info("Program <<| 0x28b |>>")


if __name__ == "__main__":
    main()
    ...
