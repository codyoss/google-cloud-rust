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

/// The Google Cloud client for the Common Types.
///
/// Defines common types for Google APIs.
library;

import 'package:google_cloud_common/common.dart';

/// Represents a textual expression in the Common Expression Language (CEL)
/// syntax. CEL is a C-like expression language. The syntax and semantics of CEL
/// are documented at https://github.com/google/cel-spec.
///
/// Example (Comparison):
///
///     title: "Summary size limit"
///     description: "Determines if a summary is less than 100 chars"
///     expression: "document.summary.size() < 100"
///
/// Example (Equality):
///
///     title: "Requestor is owner"
///     description: "Determines if requestor is the document owner"
///     expression: "document.owner == request.auth.claims.email"
///
/// Example (Logic):
///
///     title: "Public documents"
///     description: "Determine whether the document should be publicly visible"
///     expression: "document.type != 'private' && document.type != 'internal'"
///
/// Example (Data Manipulation):
///
///     title: "Notification string"
///     description: "Create a notification string with a timestamp."
///     expression: "'New message received at ' + string(document.create_time)"
///
/// The exact variables and functions that may be referenced within an expression
/// are determined by the service that evaluates it. See the service
/// documentation for additional information.
class Expr extends Message {

  /// Textual representation of an expression in Common Expression Language
  /// syntax.
  final String? expression;

  /// Optional. Title for the expression, i.e. a short string describing
  /// its purpose. This can be used e.g. in UIs which allow to enter the
  /// expression.
  final String? title;

  /// Optional. Description of the expression. This is a longer text which
  /// describes the expression, e.g. when hovered over it in a UI.
  final String? description;

  /// Optional. String indicating the location of the expression for error
  /// reporting, e.g. a file name and a position in the file.
  final String? location;

  Expr({
    this.expression,
    this.title,
    this.description,
    this.location,
  });

  factory Expr.fromJson(Map<String, dynamic> json) {
    return Expr(
      expression: json['expression'],
      title: json['title'],
      description: json['description'],
      location: json['location'],
    );
  }

  @override
  Object toJson() {
    return {
      if (expression != null) 'expression': expression,
      if (title != null) 'title': title,
      if (description != null) 'description': description,
      if (location != null) 'location': location,
    };
  }

  @override
  String toString() {
    final contents = [
      if (expression != null) 'expression=$expression',
      if (title != null) 'title=$title',
      if (description != null) 'description=$description',
      if (location != null) 'location=$location',
    ].join(',');
    return 'Expr($contents)';
  }
}
