import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:utils/src/emoji_string_manipulator.dart';
import 'package:utils/utils.dart';

void main() {
  group('colorGenerator', () {
    const testcases = <MapEntry<String, Color>>[
      MapEntry("12D3KooWKpbco5QyiKZ62CWaYrsznDBd6EpheAm25V9L4XJUwQoD",
          Color(0xff5d4037)),
      MapEntry("12D3KooWDnc9w3G99BoabkEWjgMFvHkJF7LKR9jq7SJ2E1jhcg8Q",
          Color(0xffd50000)),
      MapEntry("12D3KooWBqHe7MfH7kExsAGZ85heGbcLo1poUVEGdGPQinTCAkit",
          Color(0xff1976d2)),
      MapEntry("12D3KooWEbzJbVGua4EQNKQVUYoA46vcXnfePfm3ZL7C8pGGqDNX",
          Color(0xff0091ea)),
      MapEntry("12D3KooWHkqXTs8PYaDXN7UR2V48wvAseZ56GQc2FqfrjG7LQ4V1",
          Color(0xff2962ff)),
      MapEntry("12D3KooWReEW9zqjFj7v3QYVUBDNZtmKQ6FhYskJz2eN5M69TS4d",
          Color(0xffaeea00)),
      MapEntry("12D3KooWKezUjrTjyFd2qM4aYsAJtiDXpGcVhaLz2nYQRXhv9Gqy",
          Color(0xff0288d1)),
      MapEntry("12D3KooWRHeY7vjgt3tzcsn3ENDwQH8GphBEKTkYwmbK992XcLYM",
          Color(0xffc2185b)),
      MapEntry("12D3KooWFxdEx53gtS96BoxMbbuKNR7EhJjvBnDwPV9RCqyQz5Ae",
          Color(0xfff57c00)),
      MapEntry("12D3KooWJbdZpcfuGG7fSXYw2YSHmrg11xn1NoKbS2op8cvh66ec",
          Color(0xff00c853)),
      MapEntry("12D3KooWGB9MjCAfyhKTMZJkyCxmduazav9K7PgTxFARZjUYms72",
          Color(0xff64dd17)),
      MapEntry("12D3KooWE8FbJhrRdhuHeeNG2S1JfJvx5FV9DESpxTu2FXwVr5Ss",
          Color(0xff64dd17)),
      MapEntry("12D3KooWHJWQQVDPBqMiJ5XbbatVTi6DMUL9TTJDjDPzxFqhb29V",
          Color(0xff00b8d4)),
      MapEntry("12D3KooWJdW6aCXHZ4zAV4V7mEb2ycTBTwUFaBa1nVyFasSxJN5E",
          Color(0xff0288d1)),
      MapEntry("12D3KooWQkFfWjR5Me3UPyCh4MJjitZt6rzhpCD5Ecp1Hhyzv3hx",
          Color(0xffafb42b)),
      MapEntry("12D3KooWH8tQ58qn1sHNLNMS7n9tdtRPj3hWdNJarAkA4Uvhc9jc",
          Color(0xff2962ff)),
      MapEntry("12D3KooWEKpcqggRXbeoZCGaBX3BmLNP5jbcqXXafEmw4utP6ueV",
          Color(0xff689f38)),
      MapEntry("12D3KooWL246vs7VM3fP3RUS82YFeczLhsUmHZwr83vm38aeb9HP",
          Color(0xff00bfa5)),
      MapEntry("12D3KooWL6rhRz3nhpB75kj5yAms9yKdSYnEASFoVA6u9Tu8udPu",
          Color(0xff388e3c)),
      MapEntry("12D3KooWMxjSB1QQjMEWqqfngHsGeM64o3bAfJbeWgo5DGGaBnqo",
          Color(0xff2962ff)),
      MapEntry("12D3KooWDyQejcM8gyGV1ttYokxz8p6o6DoZqYx54zb3QPvKT8Wd",
          Color(0xff5d4037)),
      MapEntry("12D3KooWNyLEJCzF44cbM4fRNt9iNjcSLrktFQZH4QaPkjSjstY8",
          Color(0xff455a64)),
      MapEntry("12D3KooWPLxE4LkfV92JaEZqXBWKf8UqiYYq87EmewWnyeD8zahL",
          Color(0xff304ffe)),
      MapEntry("12D3KooWCSBvG3jCDPWLqynTa7ms6wRAZrWBssvvMpiDfhu3Wg2p",
          Color(0xffc51162)),
      MapEntry("12D3KooWMdgH7BHHWhbpTz75vtfDnorkLwXyv14oBdPc1PPFmfti",
          Color(0xff00796b)),
      MapEntry("12D3KooWRsFtLkW2ciCGSdBi7wiKQfSrUBcCuzs3CHJpk2pSaiJC",
          Color(0xffe64a19)),
      MapEntry("12D3KooWPNaLbxpsMd75u6WxwMdM5osBf7VdLcB2h6eoUvoJidBa",
          Color(0xffff6d00)),
      MapEntry("12D3KooWRnrxm3PZ3XaUKRjDYbc9XazxTtzEpUxFVaAdfPfhW4no",
          Color(0xff455a64)),
      MapEntry("12D3KooWEz45yVfCRmzVSnEgqrCy56MxCmtXkagNaqQ5FurL7EMQ",
          Color(0xfffbc02d)),
      MapEntry("12D3KooWGmBy3WW4dDT2mZYHsWXQW8gn2rdUwXGAR95iAeG51zY6",
          Color(0xff64dd17)),
      MapEntry("12D3KooWD4R3RWm3FaZzp9L66ywEcpFAmEPNLK6HphdwsBhgxZv8",
          Color(0xffdd2c00)),
    ];

    for (final tc in testcases) {
      test('Test n…${testcases.indexOf(tc)}: id must be ${tc.value}', () {
        expect(colorGenerationStrategy(tc.key), tc.value);
      });
    }
  });

  group('removeEmoji', () {
    String testDataWithSpace =
        ' 🤣h😌e🙄l😪l😓o😳🤔👨‍🦰🤶🏿 🧝‍♂️🍝🥘🌯🍦🥂🥂🎂🍰🧁🍨🍧😁w🤷‍♂️o😎r🤪l🤦‍♂️d🐸🤑😆😖🎉🍾🤟🤩😢🐭😡😍📧😄😔😇🧐😈🙁🤓🙂🥱🌬🌫🌨⛈⛈🌨 ';

    test('trimText = true (default) Validation', () {
      expect(removeEmoji(testDataWithSpace), 'hello world');
    });

    test('trimText = false (override) Validation', () {
      expect(removeEmoji(testDataWithSpace, '', false), ' hello world ');
    });
  });

  group('initials', () {
    const names = <MapEntry<String, String>>[
      MapEntry('Name', 'N'),
      MapEntry('Na Me', 'NM'),
      MapEntry('na me ye be', 'NB'),
      MapEntry('nameyebe', 'N'),
      MapEntry('NAME NAME MENA ANEM', 'NA'),
      MapEntry('  NAME NAME MENA ANEM', 'NA'),
      MapEntry('   NAME NAME MENA ANEM   ', 'NA'),
      MapEntry('NE ', 'N'),
      MapEntry('l🤣h😌o🙄😪😓😳ggasdf', '🤣'),
      MapEntry('😌🤣h😌o🙄😪😓😳ggasdf', '😌'),
      MapEntry('😳🤣h😌🙄😪😓😳ggasdf', '😳'),
    ];

    for (final tc in names) {
      test('${tc.key} becomes ${tc.value}', () => expect(tc.value, initials(tc.key)));
    }

    test('empty string throws', () => expect(() => initials(''), throwsAssertionError));
  });

  group('describeFuzzyTimestamp', () {
    final origin = DateTime(2000, 06, 15);

    final testcases = <MapEntry<DateTime, String>>[
      MapEntry(origin.subtract(const Duration(minutes: 5)), '5 min.'),
      MapEntry(origin.subtract(const Duration(hours: 1)), 'about an hour'),
      MapEntry(origin.subtract(const Duration(hours: 2)), '2 hours'),
      MapEntry(origin.subtract(const Duration(hours: 21)), '21 hours'),
      MapEntry(origin.subtract(const Duration(days: 4)), '4 days'),
      MapEntry(origin.subtract(const Duration(days: 40)), 'about a month'),
      MapEntry(origin.subtract(const Duration(days: 45)), 'about a month'),
      MapEntry(origin.subtract(const Duration(days: 50)), 'about a month'),
      MapEntry(origin.subtract(const Duration(days: 51)), '25 Apr 2000'),
      MapEntry(origin.subtract(const Duration(days: 60)), '16 Apr 2000'),
      MapEntry(origin.subtract(const Duration(days: 365)), '16 Jun 1999'),
      MapEntry(origin.add(const Duration(days: 1000)), 'a moment ago'),
    ];

    for (final t in testcases) {
      test('${t.key} from ORIGIN: $origin becomes `${t.value}`',
           () => expect(describeFuzzyTimestamp(t.key, clock: origin), t.value),
      );
    }
  });
}
