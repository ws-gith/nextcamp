ThisBuild / scalaVersion := "3.5.2"
ThisBuild / version := "0.1.0-SNAPSHOT"
ThisBuild / name := "nextcamp"

lazy val migrations = (project in file("migrations"))
    .settings(
      libraryDependencies ++= Seq()
    )

lazy val root = (project in file("."))
    .aggregate(migrations)
    .settings(
      libraryDependencies ++= Seq()
    )
