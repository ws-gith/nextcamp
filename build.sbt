ThisBuild / scalaVersion := "3.5.2"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / name := "nextcamp"

lazy val wp_database = (project in file("wp-database"))
    .settings(
      libraryDependencies ++= Seq()
    )

lazy val root = (project in file("."))
    .aggregate(wp_database)
    .settings(
      libraryDependencies ++= Seq()
    )
