use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let senders = get_senders( get_participants());
    let pairings =  get_pairings(senders);
    print_pairings(pairings);
}

fn get_senders(participants: Vec<&str>) -> Vec<&str>
{
    let mut senders: Vec<&str> = participants.clone();
    senders.shuffle(&mut thread_rng());
    return senders;
}

fn print_pairings(pairings: Vec<Vec<&str>>)
{
    let mut pairingIndex : usize = 0;
    for pairing in  pairings {
        println!("{:?} -> {:?}", pairing[0], pairing[1]);
        pairingIndex += 1;
    }
}

fn get_pairings(senders: Vec<&str>) -> Vec<Vec<&str>>
{
    let mut pairings: Vec<Vec<&str>> = Vec::new();
    let numParticipants = senders.len();
    let offset = get_offset(numParticipants);

    let mut senderIndex: usize = 0;
    for sender in senders.clone() {
        pairings.push(
        Vec::from( [sender, senders[calc_receiver_index(senderIndex, numParticipants, offset)]])
        );
        senderIndex += 1;
    }
    return pairings;
}


fn get_offset(numParticipants: usize) -> usize
{
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..numParticipants);
}

fn calc_receiver_index(indexSender: usize, numParticipants: usize, offset: usize) -> usize
{
    let virtualIndex = indexSender + offset;
    let adjustedIndex = virtualIndex % numParticipants;
    return adjustedIndex;
}

fn get_participants() -> Vec<&'static str> {
    let participants = vec!["Michael", "Alice", "Bob", "Charlie", "Dennis", "Eleonore", "Frank"];
    return participants;
}
