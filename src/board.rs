use geometry::line_segments::{LineSegments,LineSegmentRefs};
use geometry::line_segment::LineSegment;
use geometry::vector::Vector;
use ball::Ball;
use paddle::Paddle;
use config::Config;
use state::State;
use io::Input;
use score::Score;
use ui;

// TODO: add r_paddle to board struct
pub struct Board {
    pub ball: Ball,
    pub reflective_lines: LineSegments,
    pub l_paddle: Paddle,
    // pub r_paddle: Paddle,
}

impl Board {
    pub fn new(config: &Config) -> Board {
        let l_paddle = Paddle::new();
        let line_segments =  LineSegments::new_top_and_bottom_guards();
        // line_segments.0.push(l_paddle.line_segment);

        Board {
            ball: Ball::new(),
            reflective_lines: line_segments,
            l_paddle: l_paddle,
            // r_paddle: Paddle::new(config),
        }
    }

    pub fn update(&mut self, input: &mut Input, state: &mut State, score: &mut Score, tick_count: i32) {

        self.l_paddle.update(input);
        let paddle_line_segment = self.l_paddle.line_segment();
        state.paddle_line = paddle_line_segment;

        let mut lines = state.intersection_lines.clone();
        lines.0.push(state.paddle_line.clone());

        if input.shape_toggle {
            // potential_shape = self.update_shape();
            // for line in potential_shape.0.iter() {
            //     line_references.0.push(line);
            // }
        }

        self.ball = self.ball.update_position(&lines, input, score);
    }

    fn update_shape(&mut self) -> LineSegments {
        let vec_a = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 };
        let vec_b = Vector { x: (*ui::MAX_X / 3) as f64 * 2.0, y: (*ui::MAX_Y / 3) as f64 };
        let vec_c = Vector { x: (*ui::MAX_X / 3) as f64, y: (*ui::MAX_Y / 3) as f64 * 2.0 };

        let a = LineSegment(vec_a, vec_b);
        let b = LineSegment(vec_b, vec_c);
        let c = LineSegment(vec_c, vec_a);
        LineSegments(vec!(a, b, c))
        // line_references.0.push(&a);
    }

}
