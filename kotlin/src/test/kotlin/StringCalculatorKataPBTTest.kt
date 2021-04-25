import arrow.core.left
import arrow.core.right
import io.kotest.core.Tuple3
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.shouldBe
import io.kotest.property.Arb
import io.kotest.property.Exhaustive
import io.kotest.property.arbitrary.*
import io.kotest.property.checkAll
import io.kotest.property.exhaustive.azstring

class StringCalculatorKataPBTTest : FunSpec({

    val defaultDelimiters = Arb.element(",", "\n")
    val validValues = Arb.positiveInts(1000)

    context("step1") {
        test("should return 0 on empty string") {
            add("") shouldBe 0.right()
        }
        test("should return the value on single number") {
            validValues.checkAll { a -> add("$a") shouldBe a.right() }
        }
        test("should return the sum of two numbers") {
            Arb.bind(
                validValues,
                validValues,
                ::Pair
            ).checkAll { (a, b) -> add("$a,$b") shouldBe (a + b).right() }
        }
        test("should result InvalidNumber on a invalid int string") {
            Exhaustive.azstring(1..100).checkAll { x ->
                add(x) shouldBe InvalidNumbersError(listOf(x)).left()
            }
        }
    }
    context("step2") {
        test("should return the sum of any size of numbers") {
            Arb.list(validValues).checkAll { x ->
                add(x.joinToString(",")) shouldBe x.sum().right()
            }
        }
        test("should result InvalidNumber on the first invalid int string") {
            Arb.bind(
                validValues,
                Exhaustive.azstring(1..2),
                validValues,
                ::Tuple3
            ).checkAll { (x, y, z) ->
                add("$x, $y,$z") shouldBe InvalidNumbersError(listOf(y)).left()
            }
        }
    }
    context("step3") {
        test("should accept , and endline as number separator") {
            Arb.bind(
                validValues,
                validValues,
                defaultDelimiters,
                ::Tuple3
            ).checkAll { (x, y, separator) ->
                add("$x$separator$y") shouldBe (x + y).right()
            }
        }
        test("should not accept a string ending with a delimiter") {
            Arb.bind(
                Arb.list(validValues, 1..100),
                Arb.list(defaultDelimiters, 1..100)
            ) { values, separators ->
                values.zip(separators).joinToString { (x, y) -> "$x$y" }
            }.checkAll { x ->
                add(x) shouldBe StartingOrEndingByDelimiter.left()
            }
        }
        test("should not accept a string starting with a delimiter") {
            Arb.bind(
                Arb.list(validValues, 1..100),
                Arb.list(defaultDelimiters, 1..100)
            ) { values, separators ->
                values.zip(separators).joinToString { (x, y) -> "$y$x" }
            }.checkAll { x ->
                add(x) shouldBe StartingOrEndingByDelimiter.left()
            }
        }
    }
    context("step4") {
        test("should be able to define a delimiter") {
            Arb.bind(
                Arb.element(";", "/"),
                Arb.list(validValues),
                ::Pair
            ).checkAll { (newDelimiter, values) ->
                add("//[$newDelimiter]\n${values.joinToString(newDelimiter)}") shouldBe values.sum().right()
            }
        }
    }
    context("step5") {
        test("should not accept negative integers") {
            Arb.negativeInts().checkAll { add("$it") shouldBe NegativeInteger(listOf(it)).left() }
            Arb.bind(
                Arb.negativeInts(),
                Arb.list(validValues),
                defaultDelimiters,
            ) { invalidValue, validValues, delimiter ->
                invalidValue to (validValues + invalidValue).shuffled().map{ it.toString()}.reduce { x,  y -> "$x$delimiter$y"}
            }.checkAll { (invalidValue, value) ->
                add(value) shouldBe NegativeInteger(listOf(invalidValue)).left()
            }
        }
        test("should accumulate all negative integers") {
            Arb.negativeInts().checkAll { add("$it") shouldBe NegativeInteger(listOf(it)).left() }
            Arb.bind(
                Arb.list(Arb.negativeInts(),1..100),
                Arb.list(validValues),
                defaultDelimiters,
            ) { invalidValues, validValues, delimiter ->
                invalidValues to (validValues + invalidValues).shuffled().map{ it.toString()}.reduce { x,  y -> "$x$delimiter$y"}
            }.checkAll { (invalidValues, value) ->
                // Should import kotest assert for arrow instead of forcing order
                add(value).mapLeft { NegativeInteger((it as NegativeInteger).values.sorted()) } shouldBe NegativeInteger(invalidValues.sorted()).left()
            }
        }
    }
    context("step6") {
        test("should skip values > 1000") {
            Arb.list(Arb.positiveInts(3000)).checkAll { values ->
                add(values.joinToString(defaultDelimiters.next())) shouldBe values.filter { it <= 1000 }.sum().right()
            }
        }
    }
    context("step7") {
        test("should be able to define a delimiter with multiple characters") {
            Arb.bind(
                Arb.element("***", "__", "---"),
                Arb.list(validValues),
                ::Pair
            ).checkAll { (newDelimiter, values) ->
                add("//[$newDelimiter]\n${values.joinToString(newDelimiter)}") shouldBe values.sum().right()
            }
        }
    }
    context("step8") {
        test("should be able to define multiple delimiters") {
            Arb.bind(
                Arb.list(Arb.element("*", "_", "-"), 1..10),
                Arb.list(validValues, 1..100),
                ::Pair
            ).checkAll { (newDelimiters, values) ->
                val valueString = "//${newDelimiters.joinToString("") { "[$it]" }}\n${values.map{it.toString()}.reduce { x, y -> "$x${newDelimiters.random()}$y" }}"
                println("Test $valueString")
                add(valueString) shouldBe values.sum().right()
            }
        }
    }
    context("step9") {
        test("should be able to define multiple delimiters with multiple characters") {
            Arb.bind(
                Arb.list(Arb.element("***", "__", "---"), 1..10),
                Arb.list(validValues, 1..100),
                ::Pair
            ).checkAll { (newDelimiters, values) ->
                val valueString = "//${newDelimiters.joinToString("") { "[$it]" }}\n${values.map{it.toString()}.reduce { x, y -> "$x${newDelimiters.random()}$y" }}"
                println("Test $valueString")
                add(valueString) shouldBe values.sum().right()
            }
        }
    }
})