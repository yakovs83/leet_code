import reverse_k_group.Solution._
import reverse_k_group.ListNode
object Main extends App {
  val l = from_list(List(1, 2, 3, 4, 5, 6))
  val r = reverseKGroup(l, 4)
  println(s"$r")
}
