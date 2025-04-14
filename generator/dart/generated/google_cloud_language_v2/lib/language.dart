// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.

/// The Google Cloud client for the Cloud Natural Language API.
///
/// Provides natural language understanding technologies, such as sentiment
/// analysis, entity recognition, entity sentiment analysis, and other text
/// annotations, to developers.
library;

import 'package:google_cloud_gax/common.dart';
import 'package:google_cloud_gax/src/json_helpers.dart';
import 'package:http/http.dart' as http;

/// Provides text analysis operations such as sentiment analysis and entity
/// recognition.
class LanguageService {
  static const String _host = 'language.googleapis.com';

  final ServiceClient _client;

  LanguageService({required http.Client client})
      : _client = ServiceClient(client: client);

  /// Analyzes the sentiment of the provided text.
  Future<AnalyzeSentimentResponse> analyzeSentiment(
      AnalyzeSentimentRequest request) async {
    final url = Uri.https(_host, '/v2/documents:analyzeSentiment');
    final response = await _client.post(url, body: request);
    return AnalyzeSentimentResponse.fromJson(response);
  }

  /// Finds named entities (currently proper names and common nouns) in the text
  /// along with entity types, probability, mentions for each entity, and
  /// other properties.
  Future<AnalyzeEntitiesResponse> analyzeEntities(
      AnalyzeEntitiesRequest request) async {
    final url = Uri.https(_host, '/v2/documents:analyzeEntities');
    final response = await _client.post(url, body: request);
    return AnalyzeEntitiesResponse.fromJson(response);
  }

  /// Classifies a document into categories.
  Future<ClassifyTextResponse> classifyText(ClassifyTextRequest request) async {
    final url = Uri.https(_host, '/v2/documents:classifyText');
    final response = await _client.post(url, body: request);
    return ClassifyTextResponse.fromJson(response);
  }

  /// Moderates a document for harmful and sensitive categories.
  Future<ModerateTextResponse> moderateText(ModerateTextRequest request) async {
    final url = Uri.https(_host, '/v2/documents:moderateText');
    final response = await _client.post(url, body: request);
    return ModerateTextResponse.fromJson(response);
  }

  /// A convenience method that provides all features in one call.
  Future<AnnotateTextResponse> annotateText(AnnotateTextRequest request) async {
    final url = Uri.https(_host, '/v2/documents:annotateText');
    final response = await _client.post(url, body: request);
    return AnnotateTextResponse.fromJson(response);
  }

  /// Closes the client and cleans up any resources associated with it.
  ///
  /// Once [close] is called, no other methods should be called.
  void close() => _client.close();
}

/// Represents the input to API methods.
class Document extends Message {
  static const String fullyQualifiedName = 'google.cloud.language.v2.Document';

  /// Required. If the type is not set or is `TYPE_UNSPECIFIED`,
  /// returns an `INVALID_ARGUMENT` error.
  final Document_Type? type;

  /// The content of the input in string format.
  /// Cloud audit logging exempt since it is based on user data.
  final String? content;

  /// The Google Cloud Storage URI where the file content is located.
  /// This URI must be of the form: gs://bucket_name/object_name. For more
  /// details, see https://cloud.google.com/storage/docs/reference-uris.
  /// NOTE: Cloud Storage object versioning is not supported.
  final String? gcsContentUri;

  /// Optional. The language of the document (if not specified, the language is
  /// automatically detected). Both ISO and BCP-47 language codes are
  /// accepted.<br>
  /// [Language
  /// Support](https://cloud.google.com/natural-language/docs/languages) lists
  /// currently supported languages for each API method. If the language (either
  /// specified by the caller or automatically detected) is not supported by the
  /// called API method, an `INVALID_ARGUMENT` error is returned.
  final String? languageCode;

  Document({
    this.type,
    this.content,
    this.gcsContentUri,
    this.languageCode,
  }) : super(fullyQualifiedName);

  factory Document.fromJson(Map<String, dynamic> json) {
    return Document(
      type: decode(json['type'], Document_Type.fromJson),
      content: json['content'],
      gcsContentUri: json['gcsContentUri'],
      languageCode: json['languageCode'],
    );
  }

  @override
  Object toJson() {
    return {
      if (type != null) 'type': type!.toJson(),
      if (content != null) 'content': content,
      if (gcsContentUri != null) 'gcsContentUri': gcsContentUri,
      if (languageCode != null) 'languageCode': languageCode,
    };
  }

  @override
  String toString() {
    final contents = [
      if (type != null) 'type=$type',
      if (content != null) 'content=$content',
      if (gcsContentUri != null) 'gcsContentUri=$gcsContentUri',
      if (languageCode != null) 'languageCode=$languageCode',
    ].join(',');
    return 'Document($contents)';
  }
}

/// The document types enum.
class Document_Type extends Enum {
  /// The content type is not specified.
  static const typeUnspecified = Document_Type('TYPE_UNSPECIFIED');

  /// Plain text
  static const plainText = Document_Type('PLAIN_TEXT');

  /// HTML
  static const html = Document_Type('HTML');

  const Document_Type(super.value);

  factory Document_Type.fromJson(String json) => Document_Type(json);

  @override
  String toString() => 'Type.$value';
}

/// Represents a sentence in the input document.
class Sentence extends Message {
  static const String fullyQualifiedName = 'google.cloud.language.v2.Sentence';

  /// The sentence text.
  final TextSpan? text;

  /// For calls to `AnalyzeSentiment` or if
  /// `AnnotateTextRequest.Features.extract_document_sentiment`
  /// is set to true, this field will contain the sentiment for the sentence.
  final Sentiment? sentiment;

  Sentence({
    this.text,
    this.sentiment,
  }) : super(fullyQualifiedName);

  factory Sentence.fromJson(Map<String, dynamic> json) {
    return Sentence(
      text: decode(json['text'], TextSpan.fromJson),
      sentiment: decode(json['sentiment'], Sentiment.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (text != null) 'text': text!.toJson(),
      if (sentiment != null) 'sentiment': sentiment!.toJson(),
    };
  }

  @override
  String toString() => 'Sentence()';
}

/// Represents a phrase in the text that is a known entity, such as
/// a person, an organization, or location. The API associates information, such
/// as probability and mentions, with entities.
class Entity extends Message {
  static const String fullyQualifiedName = 'google.cloud.language.v2.Entity';

  /// The representative name for the entity.
  final String? name;

  /// The entity type.
  final Entity_Type? type;

  /// Metadata associated with the entity.
  ///
  /// For the metadata
  /// associated with other entity types, see the Type table below.
  final Map<String, String>? metadata;

  /// The mentions of this entity in the input document. The API currently
  /// supports proper noun mentions.
  final List<EntityMention>? mentions;

  /// For calls to `AnalyzeEntitySentiment` or if
  /// `AnnotateTextRequest.Features.extract_entity_sentiment`
  /// is set to true, this field will contain the aggregate sentiment expressed
  /// for this entity in the provided document.
  final Sentiment? sentiment;

  Entity({
    this.name,
    this.type,
    this.metadata,
    this.mentions,
    this.sentiment,
  }) : super(fullyQualifiedName);

  factory Entity.fromJson(Map<String, dynamic> json) {
    return Entity(
      name: json['name'],
      type: decode(json['type'], Entity_Type.fromJson),
      metadata: (json['metadata'] as Map?)?.cast(),
      mentions: decodeList(json['mentions'], EntityMention.fromJson),
      sentiment: decode(json['sentiment'], Sentiment.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (name != null) 'name': name,
      if (type != null) 'type': type!.toJson(),
      if (metadata != null) 'metadata': metadata,
      if (mentions != null) 'mentions': encodeList(mentions),
      if (sentiment != null) 'sentiment': sentiment!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (name != null) 'name=$name',
      if (type != null) 'type=$type',
    ].join(',');
    return 'Entity($contents)';
  }
}

/// The type of the entity. The table
/// below lists the associated fields for entities that have different
/// metadata.
class Entity_Type extends Enum {
  /// Unknown
  static const unknown = Entity_Type('UNKNOWN');

  /// Person
  static const person = Entity_Type('PERSON');

  /// Location
  static const location = Entity_Type('LOCATION');

  /// Organization
  static const organization = Entity_Type('ORGANIZATION');

  /// Event
  static const event = Entity_Type('EVENT');

  /// Artwork
  static const workOfArt = Entity_Type('WORK_OF_ART');

  /// Consumer product
  static const consumerGood = Entity_Type('CONSUMER_GOOD');

  /// Other types of entities
  static const other = Entity_Type('OTHER');

  /// Phone number
  ///
  /// The metadata lists the phone number, formatted according to local
  /// convention, plus whichever additional elements appear in the text:
  ///
  /// * `number` - the actual number, broken down into sections as per local
  /// convention
  /// * `national_prefix` - country code, if detected
  /// * `area_code` - region or area code, if detected
  /// * `extension` - phone extension (to be dialed after connection), if
  /// detected
  static const phoneNumber = Entity_Type('PHONE_NUMBER');

  /// Address
  ///
  /// The metadata identifies the street number and locality plus whichever
  /// additional elements appear in the text:
  ///
  /// * `street_number` - street number
  /// * `locality` - city or town
  /// * `street_name` - street/route name, if detected
  /// * `postal_code` - postal code, if detected
  /// * `country` - country, if detected
  /// * `broad_region` - administrative area, such as the state, if detected
  /// * `narrow_region` - smaller administrative area, such as county, if
  /// detected
  /// * `sublocality` - used in Asian addresses to demark a district within a
  /// city, if detected
  static const address = Entity_Type('ADDRESS');

  /// Date
  ///
  /// The metadata identifies the components of the date:
  ///
  /// * `year` - four digit year, if detected
  /// * `month` - two digit month number, if detected
  /// * `day` - two digit day number, if detected
  static const date = Entity_Type('DATE');

  /// Number
  ///
  /// The metadata is the number itself.
  static const number = Entity_Type('NUMBER');

  /// Price
  ///
  /// The metadata identifies the `value` and `currency`.
  static const price = Entity_Type('PRICE');

  const Entity_Type(super.value);

  factory Entity_Type.fromJson(String json) => Entity_Type(json);

  @override
  String toString() => 'Type.$value';
}

/// Represents the feeling associated with the entire text or entities in
/// the text.
class Sentiment extends Message {
  static const String fullyQualifiedName = 'google.cloud.language.v2.Sentiment';

  /// A non-negative number in the [0, +inf) range, which represents
  /// the absolute magnitude of sentiment regardless of score (positive or
  /// negative).
  final double? magnitude;

  /// Sentiment score between -1.0 (negative sentiment) and 1.0
  /// (positive sentiment).
  final double? score;

  Sentiment({
    this.magnitude,
    this.score,
  }) : super(fullyQualifiedName);

  factory Sentiment.fromJson(Map<String, dynamic> json) {
    return Sentiment(
      magnitude: (json['magnitude'] as num?)?.toDouble(),
      score: (json['score'] as num?)?.toDouble(),
    );
  }

  @override
  Object toJson() {
    return {
      if (magnitude != null) 'magnitude': magnitude,
      if (score != null) 'score': score,
    };
  }

  @override
  String toString() {
    final contents = [
      if (magnitude != null) 'magnitude=$magnitude',
      if (score != null) 'score=$score',
    ].join(',');
    return 'Sentiment($contents)';
  }
}

/// Represents a mention for an entity in the text. Currently, proper noun
/// mentions are supported.
class EntityMention extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.EntityMention';

  /// The mention text.
  final TextSpan? text;

  /// The type of the entity mention.
  final EntityMention_Type? type;

  /// For calls to `AnalyzeEntitySentiment` or if
  /// `AnnotateTextRequest.Features.extract_entity_sentiment`
  /// is set to true, this field will contain the sentiment expressed for this
  /// mention of the entity in the provided document.
  final Sentiment? sentiment;

  /// Probability score associated with the entity.
  ///
  /// The score shows the probability of the entity mention being the entity
  /// type. The score is in (0, 1] range.
  final double? probability;

  EntityMention({
    this.text,
    this.type,
    this.sentiment,
    this.probability,
  }) : super(fullyQualifiedName);

  factory EntityMention.fromJson(Map<String, dynamic> json) {
    return EntityMention(
      text: decode(json['text'], TextSpan.fromJson),
      type: decode(json['type'], EntityMention_Type.fromJson),
      sentiment: decode(json['sentiment'], Sentiment.fromJson),
      probability: (json['probability'] as num?)?.toDouble(),
    );
  }

  @override
  Object toJson() {
    return {
      if (text != null) 'text': text!.toJson(),
      if (type != null) 'type': type!.toJson(),
      if (sentiment != null) 'sentiment': sentiment!.toJson(),
      if (probability != null) 'probability': probability,
    };
  }

  @override
  String toString() {
    final contents = [
      if (type != null) 'type=$type',
      if (probability != null) 'probability=$probability',
    ].join(',');
    return 'EntityMention($contents)';
  }
}

/// The supported types of mentions.
class EntityMention_Type extends Enum {
  /// Unknown
  static const typeUnknown = EntityMention_Type('TYPE_UNKNOWN');

  /// Proper name
  static const proper = EntityMention_Type('PROPER');

  /// Common noun (or noun compound)
  static const common = EntityMention_Type('COMMON');

  const EntityMention_Type(super.value);

  factory EntityMention_Type.fromJson(String json) => EntityMention_Type(json);

  @override
  String toString() => 'Type.$value';
}

/// Represents a text span in the input document.
class TextSpan extends Message {
  static const String fullyQualifiedName = 'google.cloud.language.v2.TextSpan';

  /// The content of the text span, which is a substring of the document.
  final String? content;

  /// The API calculates the beginning offset of the content in the original
  /// document according to the
  /// `EncodingType` specified in the API
  /// request.
  final int? beginOffset;

  TextSpan({
    this.content,
    this.beginOffset,
  }) : super(fullyQualifiedName);

  factory TextSpan.fromJson(Map<String, dynamic> json) {
    return TextSpan(
      content: json['content'],
      beginOffset: json['beginOffset'],
    );
  }

  @override
  Object toJson() {
    return {
      if (content != null) 'content': content,
      if (beginOffset != null) 'beginOffset': beginOffset,
    };
  }

  @override
  String toString() {
    final contents = [
      if (content != null) 'content=$content',
      if (beginOffset != null) 'beginOffset=$beginOffset',
    ].join(',');
    return 'TextSpan($contents)';
  }
}

/// Represents a category returned from the text classifier.
class ClassificationCategory extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.ClassificationCategory';

  /// The name of the category representing the document.
  final String? name;

  /// The classifier's confidence of the category. Number represents how certain
  /// the classifier is that this category represents the given text.
  final double? confidence;

  /// Optional. The classifier's severity of the category. This is only present
  /// when the ModerateTextRequest.ModelVersion is set to MODEL_VERSION_2, and
  /// the corresponding category has a severity score.
  final double? severity;

  ClassificationCategory({
    this.name,
    this.confidence,
    this.severity,
  }) : super(fullyQualifiedName);

  factory ClassificationCategory.fromJson(Map<String, dynamic> json) {
    return ClassificationCategory(
      name: json['name'],
      confidence: (json['confidence'] as num?)?.toDouble(),
      severity: (json['severity'] as num?)?.toDouble(),
    );
  }

  @override
  Object toJson() {
    return {
      if (name != null) 'name': name,
      if (confidence != null) 'confidence': confidence,
      if (severity != null) 'severity': severity,
    };
  }

  @override
  String toString() {
    final contents = [
      if (name != null) 'name=$name',
      if (confidence != null) 'confidence=$confidence',
      if (severity != null) 'severity=$severity',
    ].join(',');
    return 'ClassificationCategory($contents)';
  }
}

/// The sentiment analysis request message.
class AnalyzeSentimentRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnalyzeSentimentRequest';

  /// Required. Input document.
  final Document? document;

  /// The encoding type used by the API to calculate sentence offsets.
  final EncodingType? encodingType;

  AnalyzeSentimentRequest({
    this.document,
    this.encodingType,
  }) : super(fullyQualifiedName);

  factory AnalyzeSentimentRequest.fromJson(Map<String, dynamic> json) {
    return AnalyzeSentimentRequest(
      document: decode(json['document'], Document.fromJson),
      encodingType: decode(json['encodingType'], EncodingType.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (document != null) 'document': document!.toJson(),
      if (encodingType != null) 'encodingType': encodingType!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (encodingType != null) 'encodingType=$encodingType',
    ].join(',');
    return 'AnalyzeSentimentRequest($contents)';
  }
}

/// The sentiment analysis response message.
class AnalyzeSentimentResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnalyzeSentimentResponse';

  /// The overall sentiment of the input document.
  final Sentiment? documentSentiment;

  /// The language of the text, which will be the same as the language specified
  /// in the request or, if not specified, the automatically-detected language.
  /// See `Document.language` field for more details.
  final String? languageCode;

  /// The sentiment for all the sentences in the document.
  final List<Sentence>? sentences;

  /// Whether the language is officially supported. The API may still return a
  /// response when the language is not supported, but it is on a best effort
  /// basis.
  final bool? languageSupported;

  AnalyzeSentimentResponse({
    this.documentSentiment,
    this.languageCode,
    this.sentences,
    this.languageSupported,
  }) : super(fullyQualifiedName);

  factory AnalyzeSentimentResponse.fromJson(Map<String, dynamic> json) {
    return AnalyzeSentimentResponse(
      documentSentiment: decode(json['documentSentiment'], Sentiment.fromJson),
      languageCode: json['languageCode'],
      sentences: decodeList(json['sentences'], Sentence.fromJson),
      languageSupported: json['languageSupported'],
    );
  }

  @override
  Object toJson() {
    return {
      if (documentSentiment != null)
        'documentSentiment': documentSentiment!.toJson(),
      if (languageCode != null) 'languageCode': languageCode,
      if (sentences != null) 'sentences': encodeList(sentences),
      if (languageSupported != null) 'languageSupported': languageSupported,
    };
  }

  @override
  String toString() {
    final contents = [
      if (languageCode != null) 'languageCode=$languageCode',
      if (languageSupported != null) 'languageSupported=$languageSupported',
    ].join(',');
    return 'AnalyzeSentimentResponse($contents)';
  }
}

/// The entity analysis request message.
class AnalyzeEntitiesRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnalyzeEntitiesRequest';

  /// Required. Input document.
  final Document? document;

  /// The encoding type used by the API to calculate offsets.
  final EncodingType? encodingType;

  AnalyzeEntitiesRequest({
    this.document,
    this.encodingType,
  }) : super(fullyQualifiedName);

  factory AnalyzeEntitiesRequest.fromJson(Map<String, dynamic> json) {
    return AnalyzeEntitiesRequest(
      document: decode(json['document'], Document.fromJson),
      encodingType: decode(json['encodingType'], EncodingType.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (document != null) 'document': document!.toJson(),
      if (encodingType != null) 'encodingType': encodingType!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (encodingType != null) 'encodingType=$encodingType',
    ].join(',');
    return 'AnalyzeEntitiesRequest($contents)';
  }
}

/// The entity analysis response message.
class AnalyzeEntitiesResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnalyzeEntitiesResponse';

  /// The recognized entities in the input document.
  final List<Entity>? entities;

  /// The language of the text, which will be the same as the language specified
  /// in the request or, if not specified, the automatically-detected language.
  /// See `Document.language` field for more details.
  final String? languageCode;

  /// Whether the language is officially supported. The API may still return a
  /// response when the language is not supported, but it is on a best effort
  /// basis.
  final bool? languageSupported;

  AnalyzeEntitiesResponse({
    this.entities,
    this.languageCode,
    this.languageSupported,
  }) : super(fullyQualifiedName);

  factory AnalyzeEntitiesResponse.fromJson(Map<String, dynamic> json) {
    return AnalyzeEntitiesResponse(
      entities: decodeList(json['entities'], Entity.fromJson),
      languageCode: json['languageCode'],
      languageSupported: json['languageSupported'],
    );
  }

  @override
  Object toJson() {
    return {
      if (entities != null) 'entities': encodeList(entities),
      if (languageCode != null) 'languageCode': languageCode,
      if (languageSupported != null) 'languageSupported': languageSupported,
    };
  }

  @override
  String toString() {
    final contents = [
      if (languageCode != null) 'languageCode=$languageCode',
      if (languageSupported != null) 'languageSupported=$languageSupported',
    ].join(',');
    return 'AnalyzeEntitiesResponse($contents)';
  }
}

/// The document classification request message.
class ClassifyTextRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.ClassifyTextRequest';

  /// Required. Input document.
  final Document? document;

  ClassifyTextRequest({
    this.document,
  }) : super(fullyQualifiedName);

  factory ClassifyTextRequest.fromJson(Map<String, dynamic> json) {
    return ClassifyTextRequest(
      document: decode(json['document'], Document.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (document != null) 'document': document!.toJson(),
    };
  }

  @override
  String toString() => 'ClassifyTextRequest()';
}

/// The document classification response message.
class ClassifyTextResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.ClassifyTextResponse';

  /// Categories representing the input document.
  final List<ClassificationCategory>? categories;

  /// The language of the text, which will be the same as the language specified
  /// in the request or, if not specified, the automatically-detected language.
  /// See `Document.language` field for more details.
  final String? languageCode;

  /// Whether the language is officially supported. The API may still return a
  /// response when the language is not supported, but it is on a best effort
  /// basis.
  final bool? languageSupported;

  ClassifyTextResponse({
    this.categories,
    this.languageCode,
    this.languageSupported,
  }) : super(fullyQualifiedName);

  factory ClassifyTextResponse.fromJson(Map<String, dynamic> json) {
    return ClassifyTextResponse(
      categories:
          decodeList(json['categories'], ClassificationCategory.fromJson),
      languageCode: json['languageCode'],
      languageSupported: json['languageSupported'],
    );
  }

  @override
  Object toJson() {
    return {
      if (categories != null) 'categories': encodeList(categories),
      if (languageCode != null) 'languageCode': languageCode,
      if (languageSupported != null) 'languageSupported': languageSupported,
    };
  }

  @override
  String toString() {
    final contents = [
      if (languageCode != null) 'languageCode=$languageCode',
      if (languageSupported != null) 'languageSupported=$languageSupported',
    ].join(',');
    return 'ClassifyTextResponse($contents)';
  }
}

/// The document moderation request message.
class ModerateTextRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.ModerateTextRequest';

  /// Required. Input document.
  final Document? document;

  /// Optional. The model version to use for ModerateText.
  final ModerateTextRequest_ModelVersion? modelVersion;

  ModerateTextRequest({
    this.document,
    this.modelVersion,
  }) : super(fullyQualifiedName);

  factory ModerateTextRequest.fromJson(Map<String, dynamic> json) {
    return ModerateTextRequest(
      document: decode(json['document'], Document.fromJson),
      modelVersion: decode(
          json['modelVersion'], ModerateTextRequest_ModelVersion.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (document != null) 'document': document!.toJson(),
      if (modelVersion != null) 'modelVersion': modelVersion!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (modelVersion != null) 'modelVersion=$modelVersion',
    ].join(',');
    return 'ModerateTextRequest($contents)';
  }
}

/// The model version to use for ModerateText.
class ModerateTextRequest_ModelVersion extends Enum {
  /// The default model version.
  static const modelVersionUnspecified =
      ModerateTextRequest_ModelVersion('MODEL_VERSION_UNSPECIFIED');

  /// Use the v1 model, this model is used by default when not provided.
  /// The v1 model only returns probability (confidence) score for each
  /// category.
  static const modelVersion1 =
      ModerateTextRequest_ModelVersion('MODEL_VERSION_1');

  /// Use the v2 model.
  /// The v2 model only returns probability (confidence) score for each
  /// category, and returns severity score for a subset of the categories.
  static const modelVersion2 =
      ModerateTextRequest_ModelVersion('MODEL_VERSION_2');

  const ModerateTextRequest_ModelVersion(super.value);

  factory ModerateTextRequest_ModelVersion.fromJson(String json) =>
      ModerateTextRequest_ModelVersion(json);

  @override
  String toString() => 'ModelVersion.$value';
}

/// The document moderation response message.
class ModerateTextResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.ModerateTextResponse';

  /// Harmful and sensitive categories representing the input document.
  final List<ClassificationCategory>? moderationCategories;

  /// The language of the text, which will be the same as the language specified
  /// in the request or, if not specified, the automatically-detected language.
  /// See `Document.language` field for more details.
  final String? languageCode;

  /// Whether the language is officially supported. The API may still return a
  /// response when the language is not supported, but it is on a best effort
  /// basis.
  final bool? languageSupported;

  ModerateTextResponse({
    this.moderationCategories,
    this.languageCode,
    this.languageSupported,
  }) : super(fullyQualifiedName);

  factory ModerateTextResponse.fromJson(Map<String, dynamic> json) {
    return ModerateTextResponse(
      moderationCategories: decodeList(
          json['moderationCategories'], ClassificationCategory.fromJson),
      languageCode: json['languageCode'],
      languageSupported: json['languageSupported'],
    );
  }

  @override
  Object toJson() {
    return {
      if (moderationCategories != null)
        'moderationCategories': encodeList(moderationCategories),
      if (languageCode != null) 'languageCode': languageCode,
      if (languageSupported != null) 'languageSupported': languageSupported,
    };
  }

  @override
  String toString() {
    final contents = [
      if (languageCode != null) 'languageCode=$languageCode',
      if (languageSupported != null) 'languageSupported=$languageSupported',
    ].join(',');
    return 'ModerateTextResponse($contents)';
  }
}

/// The request message for the text annotation API, which can perform multiple
/// analysis types in one call.
class AnnotateTextRequest extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnnotateTextRequest';

  /// Required. Input document.
  final Document? document;

  /// Required. The enabled features.
  final AnnotateTextRequest_Features? features;

  /// The encoding type used by the API to calculate offsets.
  final EncodingType? encodingType;

  AnnotateTextRequest({
    this.document,
    this.features,
    this.encodingType,
  }) : super(fullyQualifiedName);

  factory AnnotateTextRequest.fromJson(Map<String, dynamic> json) {
    return AnnotateTextRequest(
      document: decode(json['document'], Document.fromJson),
      features: decode(json['features'], AnnotateTextRequest_Features.fromJson),
      encodingType: decode(json['encodingType'], EncodingType.fromJson),
    );
  }

  @override
  Object toJson() {
    return {
      if (document != null) 'document': document!.toJson(),
      if (features != null) 'features': features!.toJson(),
      if (encodingType != null) 'encodingType': encodingType!.toJson(),
    };
  }

  @override
  String toString() {
    final contents = [
      if (encodingType != null) 'encodingType=$encodingType',
    ].join(',');
    return 'AnnotateTextRequest($contents)';
  }
}

/// All available features.
/// Setting each one to true will enable that specific analysis for the input.
class AnnotateTextRequest_Features extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnnotateTextRequest.Features';

  /// Optional. Extract entities.
  final bool? extractEntities;

  /// Optional. Extract document-level sentiment.
  final bool? extractDocumentSentiment;

  /// Optional. Classify the full document into categories.
  final bool? classifyText;

  /// Optional. Moderate the document for harmful and sensitive categories.
  final bool? moderateText;

  AnnotateTextRequest_Features({
    this.extractEntities,
    this.extractDocumentSentiment,
    this.classifyText,
    this.moderateText,
  }) : super(fullyQualifiedName);

  factory AnnotateTextRequest_Features.fromJson(Map<String, dynamic> json) {
    return AnnotateTextRequest_Features(
      extractEntities: json['extractEntities'],
      extractDocumentSentiment: json['extractDocumentSentiment'],
      classifyText: json['classifyText'],
      moderateText: json['moderateText'],
    );
  }

  @override
  Object toJson() {
    return {
      if (extractEntities != null) 'extractEntities': extractEntities,
      if (extractDocumentSentiment != null)
        'extractDocumentSentiment': extractDocumentSentiment,
      if (classifyText != null) 'classifyText': classifyText,
      if (moderateText != null) 'moderateText': moderateText,
    };
  }

  @override
  String toString() {
    final contents = [
      if (extractEntities != null) 'extractEntities=$extractEntities',
      if (extractDocumentSentiment != null)
        'extractDocumentSentiment=$extractDocumentSentiment',
      if (classifyText != null) 'classifyText=$classifyText',
      if (moderateText != null) 'moderateText=$moderateText',
    ].join(',');
    return 'Features($contents)';
  }
}

/// The text annotations response message.
class AnnotateTextResponse extends Message {
  static const String fullyQualifiedName =
      'google.cloud.language.v2.AnnotateTextResponse';

  /// Sentences in the input document. Populated if the user enables
  /// `AnnotateTextRequest.Features.extract_document_sentiment`.
  final List<Sentence>? sentences;

  /// Entities, along with their semantic information, in the input document.
  /// Populated if the user enables
  /// `AnnotateTextRequest.Features.extract_entities`
  /// or
  /// `AnnotateTextRequest.Features.extract_entity_sentiment`.
  final List<Entity>? entities;

  /// The overall sentiment for the document. Populated if the user enables
  /// `AnnotateTextRequest.Features.extract_document_sentiment`.
  final Sentiment? documentSentiment;

  /// The language of the text, which will be the same as the language specified
  /// in the request or, if not specified, the automatically-detected language.
  /// See `Document.language` field for more details.
  final String? languageCode;

  /// Categories identified in the input document.
  final List<ClassificationCategory>? categories;

  /// Harmful and sensitive categories identified in the input document.
  final List<ClassificationCategory>? moderationCategories;

  /// Whether the language is officially supported by all requested features.
  /// The API may still return a response when the language is not supported, but
  /// it is on a best effort basis.
  final bool? languageSupported;

  AnnotateTextResponse({
    this.sentences,
    this.entities,
    this.documentSentiment,
    this.languageCode,
    this.categories,
    this.moderationCategories,
    this.languageSupported,
  }) : super(fullyQualifiedName);

  factory AnnotateTextResponse.fromJson(Map<String, dynamic> json) {
    return AnnotateTextResponse(
      sentences: decodeList(json['sentences'], Sentence.fromJson),
      entities: decodeList(json['entities'], Entity.fromJson),
      documentSentiment: decode(json['documentSentiment'], Sentiment.fromJson),
      languageCode: json['languageCode'],
      categories:
          decodeList(json['categories'], ClassificationCategory.fromJson),
      moderationCategories: decodeList(
          json['moderationCategories'], ClassificationCategory.fromJson),
      languageSupported: json['languageSupported'],
    );
  }

  @override
  Object toJson() {
    return {
      if (sentences != null) 'sentences': encodeList(sentences),
      if (entities != null) 'entities': encodeList(entities),
      if (documentSentiment != null)
        'documentSentiment': documentSentiment!.toJson(),
      if (languageCode != null) 'languageCode': languageCode,
      if (categories != null) 'categories': encodeList(categories),
      if (moderationCategories != null)
        'moderationCategories': encodeList(moderationCategories),
      if (languageSupported != null) 'languageSupported': languageSupported,
    };
  }

  @override
  String toString() {
    final contents = [
      if (languageCode != null) 'languageCode=$languageCode',
      if (languageSupported != null) 'languageSupported=$languageSupported',
    ].join(',');
    return 'AnnotateTextResponse($contents)';
  }
}

/// Represents the text encoding that the caller uses to process the output.
/// Providing an `EncodingType` is recommended because the API provides the
/// beginning offsets for various outputs, such as tokens and mentions, and
/// languages that natively use different text encodings may access offsets
/// differently.
class EncodingType extends Enum {
  /// If `EncodingType` is not specified, encoding-dependent information (such as
  /// `begin_offset`) will be set at `-1`.
  static const none = EncodingType('NONE');

  /// Encoding-dependent information (such as `begin_offset`) is calculated based
  /// on the UTF-8 encoding of the input. C++ and Go are examples of languages
  /// that use this encoding natively.
  static const utf8 = EncodingType('UTF8');

  /// Encoding-dependent information (such as `begin_offset`) is calculated based
  /// on the UTF-16 encoding of the input. Java and JavaScript are examples of
  /// languages that use this encoding natively.
  static const utf16 = EncodingType('UTF16');

  /// Encoding-dependent information (such as `begin_offset`) is calculated based
  /// on the UTF-32 encoding of the input. Python is an example of a language
  /// that uses this encoding natively.
  static const utf32 = EncodingType('UTF32');

  const EncodingType(super.value);

  factory EncodingType.fromJson(String json) => EncodingType(json);

  @override
  String toString() => 'EncodingType.$value';
}
