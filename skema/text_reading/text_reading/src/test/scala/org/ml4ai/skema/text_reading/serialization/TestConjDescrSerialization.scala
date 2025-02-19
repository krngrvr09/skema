package org.ml4ai.skema.text_reading.serialization

import org.clulab.odin.serialization.json.MentionOps
import org.clulab.odin.{Attachment, EventMention, Mention, RelationMention, TextBoundMention}
import org.ml4ai.skema.test.ExtractionTest
import org.ml4ai.skema.text_reading.attachments.AutomatesAttachment
import org.ml4ai.skema.text_reading.mentions.CrossSentenceEventMention
import org.ml4ai.skema.text_reading.serializer.SkemaJSONSerializer


// first, let's make crossSentenceMentions to export to JSON file

class TestConjDescrSerialization extends ExtractionTest {


  val t1 = "The model consists of individuals who are either Susceptible (S), Infected (I), or Recovered (R)."
  passingTest should s"successfully serialize and deserialize Description RelationMentions that went through conjunction-related post-processing (t1): ${t1}" taggedAs (Somebody) in {
    val mentions = extractMentions(t1)
    val DescrMentions = mentions.filter(m => m.label =="Description")
    val uJson = SkemaJSONSerializer.serializeMentions(DescrMentions)
    val deserializedMentions = SkemaJSONSerializer.toMentions(uJson)
    getAttachmentJsonsFromArgs(deserializedMentions) should equal(getAttachmentJsonsFromArgs(DescrMentions))
    deserializedMentions should have size (DescrMentions.size)
    deserializedMentions.head.document.equivalenceHash should equal (DescrMentions.head.document.equivalenceHash)
    deserializedMentions.head.text should equal(DescrMentions.head.text)
    deserializedMentions.head.sentence should equal(DescrMentions.head.sentence)
    val hashesDeser = deserializedMentions.map(_.equivalenceHash)
    val hashesOrig = DescrMentions.map(_.equivalenceHash)
    hashesDeser should equal(hashesOrig)
  }

  val t2 = "while b, c and d are the removal rate of individuals in class I, IP and E respectively"
  passingTest should s"serialize and deserialize conjunction descriptions successfully from t2: ${t2}" taggedAs (Somebody) in {
    val mentions = extractMentions(t2)
    val conjDefMention = mentions.filter(m => m.labels.contains("ConjDescription"))
    val uJson = SkemaJSONSerializer.serializeMentions(conjDefMention)
    val deserializedMentions = SkemaJSONSerializer.toMentions(uJson)
    deserializedMentions should have size (conjDefMention.size)
    deserializedMentions.head.document.equivalenceHash should equal (conjDefMention.head.document.equivalenceHash)
    deserializedMentions.head.text should equal(conjDefMention.head.text)
    deserializedMentions.head.asInstanceOf[EventMention].sentence should equal(conjDefMention.head.asInstanceOf[EventMention].sentence)
    val hashesDeser = deserializedMentions.map(m => SkemaJSONSerializer.AutomatesEventMentionOps(m.asInstanceOf[EventMention]).equivalenceHash).toSet
    val hashesOrig = conjDefMention.map(m =>  SkemaJSONSerializer.AutomatesEventMentionOps(m.asInstanceOf[EventMention]).equivalenceHash).toSet
    hashesDeser should equal(hashesOrig)
  }

  val t3 = "Where S is the stock of susceptible population, I is the stock of infected, and R is the stock of recovered population."
  passingTest should s"serialize and deserialize the Description EventMentions successfully from t3: ${t3}" taggedAs (Somebody) in {
    val mentions = extractMentions(t3)
    val DescrMentions = mentions.filter(m => m.label == "Description")
    val uJson = SkemaJSONSerializer.serializeMentions(DescrMentions)
    val deserializedMentions = SkemaJSONSerializer.toMentions(uJson)
    getAttachmentJsonsFromArgs(deserializedMentions) should equal(getAttachmentJsonsFromArgs(DescrMentions))
    deserializedMentions should have size (DescrMentions.size)
    deserializedMentions.head.document.equivalenceHash should equal(DescrMentions.head.document.equivalenceHash)
    deserializedMentions.head.text should equal(DescrMentions.head.text)
    deserializedMentions.head.sentence should equal(DescrMentions.head.sentence)
    val hashesDeser = deserializedMentions.map(_.equivalenceHash)
    val hashesOrig = DescrMentions.map(_.equivalenceHash)
    hashesDeser should equal(hashesOrig)
  }

  val t4 = "where H(x) and H(y) are entropies of x and y,respectively."
  failingTest should s"serialize and deserialize the mention successfully from t4: ${t4}" taggedAs (Somebody) in {
    val mentions = extractMentions(t4)
    val conjDefMention = mentions.filter(m => m.labels.contains("ConjDescription"))
    // TODO: conjDefMention is empty which will cause an exception shortly.
    val uJson = SkemaJSONSerializer.serializeMentions(conjDefMention)
    val deserializedMentions = SkemaJSONSerializer.toMentions(uJson)
    deserializedMentions should have size (conjDefMention.size)
    deserializedMentions.head.document.equivalenceHash should equal (conjDefMention.head.document.equivalenceHash)
    deserializedMentions.head.text should equal(conjDefMention.head.text)
    deserializedMentions.head.asInstanceOf[EventMention].sentence should equal(conjDefMention.head.asInstanceOf[EventMention].sentence)
    val hashesDeser = deserializedMentions.map(m => SkemaJSONSerializer.AutomatesEventMentionOps(m.asInstanceOf[EventMention]).equivalenceHash).toSet
    val hashesOrig = conjDefMention.map(m =>  SkemaJSONSerializer.AutomatesEventMentionOps(m.asInstanceOf[EventMention]).equivalenceHash).toSet
    hashesDeser should equal(hashesOrig)
  }
}