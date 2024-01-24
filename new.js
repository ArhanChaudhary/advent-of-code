import { parse } from "node-html-parser";
import { exec as exec_ } from "child_process";
import https from "https";
import fs from "fs";
import util from "util";

let exec = util.promisify(exec_);

let day = process.argv[2];
let formatted_day = `day-${day.padStart(2, "0")}`;
let part = process.argv[3];
let year = process.argv[4] || new Date().getFullYear();

async function part1() {
  await exec(`cargo new ${formatted_day}`);
  await exec(`mkdir ${formatted_day}/src/bin`);
  await exec(`rm ${formatted_day}/src/main.rs`);

  let day_contents = await new Promise((resolve) => {
    https.get(
      `https://adventofcode.com/${year}/day/${day}`,
      (res) => {
        let data = "";
        res.on("data", (chunk) => {
          data += chunk;
        });
        res.on("end", () => {
          resolve(data);
        });
      }
    );
  });

  let root = parse(day_contents);

  let test_case = root.querySelector("pre").text;
  test_case = test_case.substring(6, test_case.length - 8);

  let test_result = root.querySelectorAll("code > em");
  test_result = test_result[test_result.length - 1].text;

  console.log(test_case);

  fs.writeFileSync(
    `${formatted_day}/src/bin/part${part}.rs`,
    fs
      .readFileSync("day_template.rs", "utf-8")
      .replace("TEST_CASE", test_case)
      .replace("TEST_RESULT", test_result)
  );

  await new Promise((resolve) => {
    let file = fs.createWriteStream(`${formatted_day}/src/bin/input.txt`);
    let buffer = [];
    https.get(
      `https://adventofcode.com/${year}/day/${day}/input`,
      {
        headers: {
          Cookie: `session=${process.env.AOC_COOKIE}`,
        },
      },
      (res) => {
        res.on("data", (chunk) => {
          buffer.push(chunk);
        });

        res.on("end", () => {
          const concatenatedBuffer = Buffer.concat(buffer);
          if (
            concatenatedBuffer.length > 0 &&
            concatenatedBuffer[concatenatedBuffer.length - 1] === 10
          ) {
            file.write(
              concatenatedBuffer.subarray(0, concatenatedBuffer.length - 1)
            );
          } else {
            file.write(concatenatedBuffer);
          }
          file.end();
          file.on("finish", () => {
            file.close();
            resolve();
          });
        });
      }
    );
  });

  try {
    await exec(`cd ${formatted_day} && cargo build`);
    await exec(`git add ${formatted_day}`);
    await exec(`git commit -m "day ${day} rust setup"`);
  } catch (e) {
    console.log(e.message);
  }
}

async function part2() {
  let day_contents = await new Promise((resolve) => {
    https.get(
      `https://adventofcode.com/${year}/day/${day}`,
      {
        headers: {
          Cookie: `session=${process.env.AOC_COOKIE}`,
        },
      },
      (res) => {
        let data = "";
        res.on("data", (chunk) => {
          data += chunk;
        });
        res.on("end", () => {
          resolve(data);
        });
      }
    );
  });

  let root = parse(day_contents);

  let test_case = root.querySelector("pre").text;
  test_case = test_case.substring(6, test_case.length - 8);

  let test_result = root.querySelectorAll("code > em");
  test_result = test_result[test_result.length - 1].text;

  let part2 = fs.readFileSync(`${formatted_day}/src/bin/part1.rs`, "utf-8");
  part2 = part2.replaceAll("part1", "part2");
  part2 = part2.replace(/(assert_eq!\(result, ).*(\);)/, `$1${test_result}$2`);
  fs.writeFileSync(`${formatted_day}/src/bin/part2.rs`, part2);
}

if (part == "1") {
  part1();
} else if (part == "2") {
  part2();
}
