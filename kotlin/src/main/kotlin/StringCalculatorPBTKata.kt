import arrow.core.*
import arrow.typeclasses.Semigroup

val customDelimiterPattern = """//(\[[^\d]+\])+\n(.*)""".toRegex()

fun add(numbers: String, delimiters: List<String> = listOf(",", "\n")): Either<AddErrors, Int> =
    customDelimiterPattern.find(numbers)?.run {
        add(groupValues.last(), groupValues[1].split("[", "]").filter(String::isNotEmpty))
    } ?: when {
        numbers.isEmpty() -> 0.right()
        delimiters.any { numbers.startsWith(it) || numbers.endsWith(it) } -> StartingOrEndingByDelimiter.left()
        else ->
            numbers.split(*delimiters.toTypedArray()).map(String::trim).right()
                .flatMap(::onlyInts)
                .flatMap(::onlyPositiveInts)
                .map { it.filter { v -> v <= 1000 } }
                .map { it.sum() }
    }

// In fact we should during a refactoring change tests to test these functions

fun onlyInts(values: List<String>): Either<InvalidNumbersError, List<Int>> =
    values.traverseValidated(InvalidNumbersErrorSemigroup) {
        it.toIntOrNull()?.valid() ?: InvalidNumbersError(listOf(it)).invalid()
    }.toEither()

fun onlyPositiveInts(values: List<Int>): Either<NegativeInteger, List<Int>> =
    values.traverseValidated(NegativeIntegerSemigroup) {
        if (it >= 0) it.valid()
        else NegativeInteger(listOf(it)).invalid()
    }.toEither()

sealed class AddErrors
data class InvalidNumbersError(val values: List<String>) : AddErrors()
object StartingOrEndingByDelimiter : AddErrors()
data class NegativeInteger(val values: List<Int>) : AddErrors()

object InvalidNumbersErrorSemigroup : Semigroup<InvalidNumbersError> {
    override fun InvalidNumbersError.combine(b: InvalidNumbersError): InvalidNumbersError =
        InvalidNumbersError(values + b.values)
}

object NegativeIntegerSemigroup : Semigroup<NegativeInteger> {
    override fun NegativeInteger.combine(b: NegativeInteger): NegativeInteger =
        NegativeInteger(values + b.values)
}