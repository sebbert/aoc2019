let rec partialDigits acc num =
  let nextDigit = num % 10
  let remaining = num / 10
  let digits = nextDigit :: acc
  if remaining > 0 then partialDigits digits remaining
  else digits

let digits = partialDigits []

let tupled2 fn (a, b) = fn a b

type ScanState =
| First
| Step of int * int * bool
| Invalid

[<EntryPoint>]
let main argv =
  let lower = 197487
  let higher = 673251

  let rangeAsDigits = seq { lower .. higher } |> Seq.map digits

  let countValidNumbers isValid =
    rangeAsDigits
    |> Seq.filter isValid
    |> Seq.length

  let part1 =
    let isValid digits =
      let pairs = Seq.pairwise digits
      let isIncreasing = pairs |> Seq.forall (tupled2 (<=))
      let hasAdjacentDigits = pairs |> Seq.exists (tupled2 (=))
      isIncreasing && hasAdjacentDigits

    countValidNumbers isValid

  printfn "Part 1: %i" part1

  // I thought this was going to be sooo pretty, but as you can see, things did not turn out that way
  let part2 =
    let foldState state next =
      match state with
      | Invalid -> Invalid
      | Step (prev, _, _)            when prev > next -> Invalid
      | Step (prev, streak, hasPair) when prev = next -> Step (next, streak + 1, hasPair)
      | Step (prev, streak, hasPair)                  -> Step (next, 1, hasPair || streak = 2)
      | First -> Step(next, 1, false)

    let isValid digits =
      digits
      |> Seq.fold foldState First
      |> function
        | Step(_, streak, hasPair) -> hasPair || streak = 2
        | _ -> false

    countValidNumbers isValid

  printfn "Part 2: %i" part2

  0
