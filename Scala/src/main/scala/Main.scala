import regexp_matching.Solution.isMatch
object Main extends App {
  val m = isMatch("aab", "c*a*b")
  println(s"It is ${if (m) "a match" else "not a match"}")
}
